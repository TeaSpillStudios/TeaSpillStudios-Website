use std::fs;

use axum::{response::Html, routing::get, Router};
use markdown::to_html;

#[tokio::main]
async fn main() {
    // Build a new app
    let app = Router::new().route(
        "/",
        get(|| async {
            let markdown = fs::read_to_string("./resources/index.md").unwrap();
            let css = fs::read_to_string("./resources/styles.css").unwrap();
            let html = to_html(&markdown);

            let css_and_html = format!(
                "<!DOCTYPE html>\n\n<head>\n<style>\n{}</style>\n</head>\n\n<body>\n{}</body>",
                css, html
            );

            Html(css_and_html)
        }),
    );

    // Bind it on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
