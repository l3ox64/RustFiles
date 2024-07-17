mod backend;

use axum::{
    routing::get,
    Router,
    response::Html,
    http::StatusCode,
};

use std::{
    fs::File,
    io::Read,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/files/*path", get(backend::files_mng::serve_directory))
        .route("/download/*path", get(backend::files_mng::download_file)); 

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Result<Html<String>, (StatusCode, String)> {
    // Serve root HTML file
    let mut file = match File::open("src/frontend/index.html") {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "File not found".to_string())),
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file".to_string()));
    }

    // Get the HTML list of files in the /files directory
    let file_list_html = backend::files_mng::file_list("files");
    let html_content = contents.replace("{{file_list}}", &file_list_html);

    Ok(Html(html_content))
}