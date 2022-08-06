use axum::{
    extract::Extension, extract::Query, response::Html, response::Json, routing::get, Router,
};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

struct State {
    counter: i32,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(Mutex::new(State { counter: 1 }));

    let app = Router::new()
        .route("/", get(handler_index))
        .route("/json", get(json))
        .route("/rand", get(handler_rand))
        .route("/add", get(handler_add))
        .route("/sub", get(handler_sub))
        .layer(Extension(shared_state));

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

async fn json() -> Json<Value> {
    Json(json!({"data": 42}))
}

async fn handler_add(Extension(state): Extension<Arc<Mutex<State>>>) -> Json<Value> {
    let mut state = state.lock().await;
    *state = State {
        counter: state.counter + 1,
    };
    Json(json!({"result(add)": state.counter}))
}

async fn handler_sub(Extension(state): Extension<Arc<Mutex<State>>>) -> Json<Value> {
    let mut state = state.lock().await;
    *state = State {
        counter: state.counter - 1,
    };
    Json(json!({"result(sub)": state.counter}))
}
