use axum::{response::Html, routing::get, Router, Server};
use std::fs;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(handler));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<String> {
    Html(fs::read_to_string("static/index.html").expect("Failed to load HTML"))
}
