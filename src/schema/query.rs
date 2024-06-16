use async_graphql::{Context, Object};

use crate::schema::models::Book;

pub struct Query;

#[Object]
impl Query {
    async fn book(&self, _ctx: &Context<'_>) -> Book {
        Book {
            title: "The Catcher in the Rye".to_string(),
            author: "J.D. Salinger".to_string(),
        }
    }
}
