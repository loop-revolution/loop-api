use super::auth::auth_payload::{require_token, validate_token};
use crate::{block_logic::block::Block, graphql::Context, Error};
use block_tools::{
	dsl::prelude::*,
	models::{BlockD, UserD},
	schema::{blocks, users},
};
use juniper::graphql_object;
pub struct User {
	/// Auto-incrementing unique ID for a user
	pub id: i32,
	/// Unique alphanumeric username for easy identification
	pub username: String,
}

#[graphql_object(context = Context)]
impl User {
	/// How many users there are in the database
	async fn credits(&self, context: &Context) -> Result<Option<i32>, Error> {
		let conn = &context.pool.get()?;
		let token = require_token(context)?;

		if &self.id != &validate_token(token)? {
			return Ok(None);
		}

		Ok(Some(
			users::dsl::users
				.filter(users::id.eq(&self.id))
				.select(users::credits)
				.first(conn)?,
		))
	}

	fn id(&self) -> i32 {
		self.id
	}

	fn username(&self) -> String {
		self.username.clone()
	}

	async fn blocks(&self, context: &Context) -> Result<Vec<Block>, Error> {
		let conn = &context.pool.get()?;

		let blocks: Vec<Block> = blocks::dsl::blocks
			.filter(blocks::dsl::owner_id.eq(self.id))
			.load::<BlockD>(conn)?
			.iter()
			.map(|blockd| Block::from(blockd))
			.collect();

		Ok(blocks)
	}
}

impl From<UserD> for User {
	fn from(userd: UserD) -> Self {
		User {
			id: userd.id,
			username: userd.username,
		}
	}
}