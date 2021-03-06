pub mod blocks;
pub mod graphql;
pub mod notifications;
pub mod sentry;
pub mod users;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

/// Generates a random string of length provided
pub fn rand_string(length: usize) -> String {
	let mut rng = thread_rng();
	iter::repeat(())
		.map(|()| rng.sample(Alphanumeric))
		.map(char::from)
		.take(length)
		.collect()
}

#[cfg(test)]
pub mod tests {
	use crate::{
		graphql::ContextData,
		rand_string,
		users::info::{password::hash_pwd, username::localize_username},
	};
	use async_graphql::{Name, Request, Value};
	use block_tools::{
		dsl,
		dsl::prelude::*,
		models::{NewUser, User},
		schema::users,
		PgConnect, PostgresPool,
	};
	use std::collections::BTreeMap;

	/// Expect a value to be an object treee, then return the tree
	pub fn expect_tree(value: &Value) -> &BTreeMap<Name, Value> {
		if let Value::Object(tree) = value {
			return tree;
		} else {
			panic!();
		}
	}

	/// Expect a tree to have a key
	pub fn expect_key<'a>(tree: &'a BTreeMap<Name, Value>, name: &str) -> &'a Value {
		match tree.get(name) {
			Some(val) => return val,
			None => panic!(),
		}
	}

	/// Expects a value to be an object, and returns a value from it
	pub fn expect_tree_val<'a>(value: &'a Value, name: &str) -> &'a Value {
		expect_key(expect_tree(value), name)
	}

	/// Builds a request from context and query
	pub fn build_request(query: String, pool: PostgresPool, token: Option<String>) -> Request {
		let mut request = Request::new(query);
		request = request.data(ContextData {
			pool,
			auth_token: token,
		});
		request
	}

	/// For parsing string Values
	pub fn rem_first_and_last(value: &str) -> &str {
		let mut chars = value.chars();
		chars.next();
		chars.next_back();
		chars.as_str()
	}

	/// For parsing string Values
	pub fn test_user(conn: &PgConnect) -> (User, String) {
		let username = rand_string(10);
		let password = rand_string(10);
		let hash = hash_pwd(password.clone()).unwrap();
		let localuname = localize_username(&username);

		let new_user = NewUser {
			username,
			localuname,
			password: hash,
			email: "fake@e.mail".into(),
			credits: 0,
			display_name: None,
		};

		(
			dsl::insert_into(users::table)
				.values(&new_user)
				.get_result(conn)
				.unwrap(),
			password,
		)
	}
}
