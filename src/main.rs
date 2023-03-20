use axum::{response::Html, routing::get, Router, Server};
use markdown::file_to_html;
use std::{fs, path::Path};
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
    let html = fs::read_to_string("static/index.html").expect("Failed to load HTML");
    let markdown = file_to_html(Path::new("static/index.md"))
        .expect("Failed to convert Markdown to HTML")
        .lines()
        .map(|x| format!("    {x}\n"))
        .collect::<String>();

    let combination = format!("{html}\n<body>\n{markdown}</body>");

    Html(combination)
}
