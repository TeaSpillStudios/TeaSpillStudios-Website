use axum::{response::Html, routing::get, Router, Server};
use markdown::file_to_html;
use std::{fs, path::Path};
use tower_http::{services::ServeDir, trace::TraceLayer};

const DEV_MODE: bool = true;

#[tokio::main]
async fn main() {
    let dir_path = if DEV_MODE {
        "static/"
    } else {
        "/web-resources/static/"
    };

    let app = Router::new()
        .nest_service(dir_path, ServeDir::new("static"))
        .route("/", get(handler));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<String> {
    let html_path = if DEV_MODE {
        "static/index.html"
    } else {
        "/web-resources/index.html"
    };
    let footer_html_path = if DEV_MODE {
        "static/index.html"
    } else {
        "/web-resources/index.html"
    };
    let markdown_path = if DEV_MODE {
        "static/index.html"
    } else {
        "/web-resources/index.html"
    };

    let html = fs::read_to_string(html_path).expect("Failed to load HTML");
    let footer_html = fs::read_to_string(footer_html_path)
        .expect("Failed to load footer HTML")
        .lines()
        .map(|x| format!("    {x}\n"))
        .collect::<String>();

    let markdown = file_to_html(Path::new(markdown_path))
        .expect("Failed to convert Markdown to HTML")
        .lines()
        .map(|x| format!("    {x}\n"))
        .collect::<String>();

    let combination = format!("{html}\n<body>\n{markdown}\n{footer_html}</body>");

    Html(combination)
}
