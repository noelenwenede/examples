use async_graphql::http::{playground_source, GQLResponse};
use async_graphql::{EmptyMutation, EmptySubscription, QueryBuilder, Schema};
use starwars::{QueryRoot, StarWars};
use std::convert::Infallible;
use warp::{http::Response, Filter, Reply};

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.execute(&schema).await;
            Ok::<_, Infallible>(warp::reply::json(&GQLResponse(resp)).into_response())
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source("/", None))
    });

    let routes = graphql_post.or(graphql_playground);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
