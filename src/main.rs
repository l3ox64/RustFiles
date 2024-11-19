mod backend;

use backend::session::session::{get_session, create_session};
use backend::file::files_mng;
use axum::{
    routing::get,
    Router,
    response::Html,
    http::StatusCode,
    extract::{State, Extension},
};
use tower_cookies::{CookieManagerLayer, Cookies};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs;

#[derive(Clone)]
struct AppState {
    session_counter: Arc<Mutex<usize>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        session_counter: Arc::new(Mutex::new(0)),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/files/*path", get(files_mng::dir_list))
        .route("/download/*path", get(files_mng::download_file))
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("Failed to start server");
}

async fn root(cookies: Cookies, State(state): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let session_id = get_session(&cookies);

    if session_id.is_none() {
        let new_session_id = create_session(&mut cookies.clone());
        let mut counter = state.session_counter.lock().await;
        *counter += 1;
        println!("Nuova sessione creata: {}, Totale sessioni: {}", new_session_id, counter);
    } else {
        println!("Sessione esistente: {}", session_id.unwrap());
    }

    let contents = read_file("src/frontend/index.html").await?;
    
    let file_list_html = files_mng::file_list("files")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let html_content = contents.replace("{{file_list}}", &file_list_html);

    Ok(Html(html_content))
}

async fn read_file(path: &str) -> Result<String, (StatusCode, String)> {
    fs::read_to_string(path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read file '{}': {}", path, e)))
}
