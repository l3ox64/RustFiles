mod backend;

use backend::session::session::{get_session, create_session}; // Importing functions directly
use backend::file::files_mng;

use axum::{
    routing::get,
    Router,
    response::Html,
    http::StatusCode,
};
use tower_cookies::{CookieManagerLayer, Cookies};
use std::fs;
use backend::file::files_mng::ServerError;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/files/*path", get(files_mng::dir_list))
        .route("/download/*path", get(files_mng::download_file))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("Failed to start server");
}

async fn root(cookies: Cookies) -> Result<Html<String>, (StatusCode, String)> {
    let session_id = get_session(&cookies); // Directly calling the imported function

    if session_id.is_none() {
        let new_session_id = create_session(&mut cookies.clone());
        println!("Nuova sessione creata: {}", new_session_id);
    }

    let contents = fs::read_to_string("src/frontend/index.html")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read file: {}", e)))?;

    let file_list_html = files_mng::file_list("files")
        .map_err(|e: ServerError| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let html_content = contents.replace("{{file_list}}", &file_list_html);

    if let Some(sid) = session_id {
        println!("Sessione esistente: {}", sid);
    }

    Ok(Html(html_content))
}
