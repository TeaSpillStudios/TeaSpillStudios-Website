use axum::{response::Html, routing::get, Router};
use std::{fs, path::Path};

async fn handler() -> Html<String> {
    let html = fs::read_to_string("./resources/index.html").expect("Failed to load `index.html`");
    let markdown = markdown::file_to_html(Path::new("./resources/index.md")).unwrap();

    Html(html + &markdown + "</body>")
}

#[tokio::main]
async fn main() {
    // Build a new app
    let app = Router::new().route("/", get(handler));

    // Bind it on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
