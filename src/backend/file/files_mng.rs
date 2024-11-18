use axum::{
    response::{Html, IntoResponse},
    extract::Path,
    http::{StatusCode, header::CONTENT_DISPOSITION},
    response::Response,
};
use std::{
    fs::{self, File},
    io::{Read, Error as IoError},
    path::Path as StdPath,
    fmt,
};

#[derive(Debug)]
pub enum ServerError {
    DirectoryNotFound(String),
    TemplateNotFound,
    FileNotFound(String),
    FileOpenError(IoError),
    FileReadError,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::DirectoryNotFound(dir) => write!(f, "Directory not found: {}", dir),
            ServerError::TemplateNotFound => write!(f, "Template file not found"),
            ServerError::FileNotFound(file) => write!(f, "File not found: {}", file),
            ServerError::FileOpenError(e) => write!(f, "Failed to open file: {}", e),
            ServerError::FileReadError => write!(f, "Failed to read file"),
        }
    }
}



pub async fn dir_list(Path(path): Path<String>) -> Result<Html<String>, (StatusCode, String)> {
    let directory = format!("files/{}", path);
    let file_list_html = file_list(&directory)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let html_template = fs::read_to_string("src/frontend/dir_list.html")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ServerError::TemplateNotFound.to_string()))?;

    let html_content = html_template
        .replace("{{directory}}", &directory)
        .replace("{{file_list}}", &file_list_html);

    Ok(Html(html_content))
}



pub fn file_list(directory: &str) -> Result<String, ServerError> {
    let paths = fs::read_dir(directory)
        .map_err(|_| ServerError::DirectoryNotFound(directory.to_string()))?;

    let mut file_list_html = String::new();

    for entry in paths {
        let entry = entry.map_err(|_| ServerError::DirectoryNotFound(directory.to_string()))?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        let file_type = if entry.file_type().map_err(ServerError::FileOpenError)?.is_dir() {
            "directory"
        } else {
            "file"
        };

        let html_entry = if file_type == "directory" {
            let file_path = format!("{}/{}", directory.strip_prefix("files/").unwrap_or(directory), file_name);
            format!(
                r#"<li class="{}">
                    <div class="file-entry">
                        <button class="nav-button" onclick="location.href='/{}'">{}</button>
                    </div>
                </li>"#,
                file_type, file_path, file_name
            )
        } else {
            let file_path = format!("{}/{}", directory.strip_prefix("files/").unwrap_or(directory), file_name);
            format!(
                r#"<li class="{}">
                    <div>
                        <span>{}</span>
                        <a href="/download/{}" class="nav-button">Download</a>
                    </div>
                </li>"#,
                file_type, file_name, file_path
            )
        };

        file_list_html.push_str(&html_entry);
    }

    Ok(file_list_html)
}



pub async fn download_file(Path(path): Path<String>) -> impl IntoResponse {
    let file_path = format!("files/{}", path);
    
    match File::open(&file_path) {
        Ok(mut file) => {
            let mut contents = Vec::new();
            if file.read_to_end(&mut contents).is_err() {
                return (StatusCode::INTERNAL_SERVER_ERROR, ServerError::FileReadError.to_string()).into_response();
            }

            let file_name = StdPath::new(&file_path).file_name().unwrap().to_string_lossy().to_string();
            let disposition = format!("attachment; filename=\"{}\"", file_name);

            Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_DISPOSITION, disposition)
                .body(contents.into())
                .unwrap()
        },
        Err(_) => (StatusCode::NOT_FOUND, ServerError::FileNotFound(file_path).to_string()).into_response(),
    }
}