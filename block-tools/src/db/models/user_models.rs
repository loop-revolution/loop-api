use diesel::PgConnection;

use super::super::schema::{potential_users, users};
use crate::diesel::*;
use crate::LoopError;
use std::time::SystemTime;

#[derive(Queryable, Clone)]
pub struct User {
	/// Auto-incrementing unique ID for a user
	pub id: i32,
	/// Unique alphanumeric username for easy identification
	pub username: String,
	/// Username (standardized)
	pub localuname: String,
	pub password: String,
	pub email: String,
	pub credits: i32,
	pub display_name: Option<String>,
	pub root_id: Option<i64>,
	pub featured_id: Option<i64>,
	pub expo_tokens: Vec<String>,
	// The latest update that the user has seen
	pub latest_update_seen_id: Option<i32>,
}

impl User {
	pub fn by_id(user_id: i32, conn: &PgConnection) -> Result<Option<Self>, LoopError> {
		Ok(users::dsl::users
			.filter(users::id.eq(user_id))
			.limit(1)
			.get_result(conn)
			.optional()?)
	}

	pub fn update_username(
		&self,
		new_username: &str,
		new_localname: &str,
		new_credits: i32,
		conn: &PgConnection,
	) -> Result<User, LoopError> {
		Ok(
			diesel::update(users::dsl::users.filter(users::id.eq(self.id)))
				.set((
					users::username.eq(new_username),
					users::localuname.eq(new_localname),
					users::credits.eq(new_credits),
				))
				.get_result(conn)?,
		)
	}

	pub fn update_display_name(
		&self,
		new_display_name: &str,
		conn: &PgConnection,
	) -> Result<User, LoopError> {
		Ok(
			diesel::update(users::dsl::users.filter(users::id.eq(self.id)))
				.set((users::display_name.eq(new_display_name),))
				.get_result(conn)?,
		)
	}

	pub fn update_password(&self, hash: &str, conn: &PgConnection) -> Result<User, LoopError> {
		Ok(
			diesel::update(users::dsl::users.filter(users::id.eq(self.id)))
				.set((users::password.eq(hash),))
				.get_result(conn)?,
		)
	}

	pub fn update_root(
		&self,
		block_id: Option<i64>,
		conn: &PgConnection,
	) -> Result<User, LoopError> {
		Ok(
			diesel::update(users::dsl::users.filter(users::id.eq(self.id)))
				.set((users::root_id.eq(block_id),))
				.get_result(conn)?,
		)
	}

	pub fn update_featured(
		&self,
		block_id: Option<i64>,
		conn: &PgConnection,
	) -> Result<User, LoopError> {
		Ok(
			diesel::update(users::dsl::users.filter(users::id.eq(self.id)))
				.set((users::featured_id.eq(block_id),))
				.get_result(conn)?,
		)
	}

	pub fn update_expo_tokens(
		&self,
		tokens: Vec<String>,
		conn: &PgConnection,
	) -> Result<User, LoopError> {
		Ok(
			diesel::update(users::dsl::users.filter(users::id.eq(self.id)))
				.set((users::expo_tokens.eq(tokens),))
				.get_result(conn)?,
		)
	}

	pub fn add_expo_token(&self, token: String, conn: &PgConnection) -> Result<User, LoopError> {
		let mut all_tokens = self.expo_tokens.clone();
		all_tokens.push(token);
		Ok(
			diesel::update(users::dsl::users.filter(users::id.eq(self.id)))
				.set((users::expo_tokens.eq(all_tokens),))
				.get_result(conn)?,
		)
	}
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
	/// Username must be unique
	pub username: String,
	/// Localized username, must be unique
	pub localuname: String,
	/// Will be securely stored
	pub password: String,
	pub email: String,
	pub credits: i32,
	pub display_name: Option<String>,
}

#[derive(Queryable)]
pub struct PotentialUser {
	pub id: i32,
	pub email: String,
	pub session_code: String,
	pub verification_code: String,
	pub username: String,
	pub password: String,
	pub created_at: SystemTime,
	pub display_name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "potential_users"]
pub struct NewPotentialUser<'a> {
	pub email: &'a str,
	pub session_code: &'a str,
	pub verification_code: &'a str,
	pub username: &'a str,
	pub password: &'a str,
	pub created_at: SystemTime,
	pub display_name: Option<String>,
}
