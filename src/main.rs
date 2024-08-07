mod backend;

use axum::{
    routing::get,
    Router,
    response::Html,
    http::StatusCode,
};
use std::fs;
use backend::files_mng::ServerError;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/files/*path", get(backend::files_mng::serve_directory))
        .route("/download/*path", get(backend::files_mng::download_file));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await
        .expect("Failed to bind to port 3000");
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await
        .expect("Failed to start server");
}

async fn root() -> Result<Html<String>, (StatusCode, String)> {
    let contents = fs::read_to_string("src/frontend/index.html")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read file: {}", e)))?;

    let file_list_html = backend::files_mng::file_list("files")
        .map_err(|e: ServerError| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let html_content = contents.replace("{{file_list}}", &file_list_html);
    Ok(Html(html_content))
}