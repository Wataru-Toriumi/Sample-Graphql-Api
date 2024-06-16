use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Book {
    pub title: String,
    pub author: String,
}
