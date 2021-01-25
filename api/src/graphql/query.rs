use super::context::Context;
use crate::{
	block_logic::{
		block::Block,
		block_queries::{block_by_id, creation_display},
	},
	user_logic::{
		auth::auth_payload::{require_token, validate_token},
		user::User,
	},
	Error,
};
use block_tools::{dsl::prelude::*, models::UserD, schema::users};
use juniper::{graphql_object, FieldResult};

/// Struct for Juniper to take Query resolvers
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
	/// How many users there are in the database
	async fn user_count(context: &Context) -> FieldResult<i32> {
		let conn = &context.pool.get()?;

		let num: i64 = users::dsl::users.count().get_result(conn)?;
		Ok(num as i32)
	}

	/// Tries to find a user with a matching ID. Will be null if a user is not found.
	async fn user_by_id(context: &Context, id: i32) -> Result<Option<User>, Error> {
		user_by_id(context, id).await
	}

	/// Tries to find a block with a matching ID. Will be null if a block is not found.
	async fn block_by_id(context: &Context, id: i32) -> Result<Option<Block>, Error> {
		block_by_id(context, id).await
	}

	/// Returns a `User` object corresponding with the authorization token.
	async fn whoami(context: &Context) -> Result<Option<User>, Error> {
		let token = require_token(context)?;
		let user_id = validate_token(token)?;

		user_by_id(context, user_id).await
	}

	pub async fn block_creation_display(
		context: &Context,
		r#type: String,
	) -> Result<String, Error> {
		creation_display(context, r#type).await
	}
}

pub async fn user_by_id(context: &Context, id: i32) -> Result<Option<User>, Error> {
	let conn = &context.pool.get()?;

	let usr: Option<UserD> = users::dsl::users
		.filter(users::id.eq(id))
		.limit(1)
		.get_result(conn)
		.optional()?;

	match usr {
		None => Ok(None),
		Some(usr) => Ok(Some(User::from(usr))),
	}
}