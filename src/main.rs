use std::fs;

use axum::{response::Html, routing::get, Router};
use markdown::to_html;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(|| async {
            let markdown = fs::read_to_string("./index.md");
            let html = to_html(&markdown.unwrap());

            let combined = format!(
                "<head>\n<style>\n{}\n</style></head>",
                fs::read_to_string("./styles.css").unwrap()
            ) + &format!("<body>{}</body>", &html);

            Html(combined)
        }),
    );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
