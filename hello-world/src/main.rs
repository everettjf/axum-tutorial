use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler_index))
        .route("/rand", get(handler_rand));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

async fn handler_rand(Query(range): Query<RangeParameters>) -> Html<String> {
    let random_number = thread_rng().gen_range(range.start..range.end);

    Html(format!("<h1>Random number: {}</h1>", random_number))
}
