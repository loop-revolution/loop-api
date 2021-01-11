#[macro_use]
extern crate diesel;

pub mod db;
pub mod graphql;
pub use graphql::Error;
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
mod test {
	use crate::{
		db::{env_db, get_pool},
		graphql::{create_schema, Context, Schema},
		Error,
	};
	use juniper::{DefaultScalarValue, ExecutionError, Value, Variables};
	use std::time::SystemTime;

	/// Generates a "random" username based on time
	pub fn rand_username() -> Result<String, Error> {
		Ok(SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)?
			.as_millis()
			.to_string())
	}

	/// Makes a Schema & Context instance for integration tests
	pub fn gen_exec(token: Option<String>) -> (Context, Schema) {
		let schema = create_schema();
		let pool = get_pool(&env_db());
		let context = Context {
			pool: pool.clone(),
			auth_token: token,
		};
		(context, schema)
	}

	pub async fn easy_exec<'a>(
		query: &'a str,
		pair: (&Schema, &Context),
	) -> (
		Value<DefaultScalarValue>,
		Vec<ExecutionError<DefaultScalarValue>>,
	) {
		juniper::execute(query, None, pair.0, &Variables::new(), pair.1)
			.await
			.unwrap()
	}
}