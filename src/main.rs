use actix_web::{guard, web, App, HttpServer};
#[allow(unused_imports)]
use async_graphql::{Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod schema;

async fn graphql_handler(
    schema: web::Data<schema::MySchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = schema::create_schema();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler),
            )
            .service(
                web::resource("/playground")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}