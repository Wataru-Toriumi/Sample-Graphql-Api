use async_graphql::{Schema, EmptyMutation, EmptySubscription};

mod query;
mod models;

pub use query::Query;

pub type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> MySchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish()
}
