use axum::{
    response::{Html, IntoResponse},
    extract::Path,
    http::{StatusCode, header::CONTENT_DISPOSITION},
    response::Response,
};
use std::{
    fs::{self, File},
    io::{Read},
    path::Path as StdPath,
};



pub async fn serve_directory(Path(path): Path<String>) -> Result<Html<String>, (StatusCode, String)> {
    let directory = format!("files/{}", path);

    // Check if the directory exists
    if !StdPath::new(&directory).exists() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Directory not found".to_string()));
    }

    // Read the directory contents
    let file_list_html = file_list(&directory);

    // Read the external HTML file
    let mut file = match File::open("src/frontend/dir_list.html") {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "Template file not found".to_string())),
    };

    let mut html_template = String::new();
    if let Err(_) = file.read_to_string(&mut html_template) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to read template file".to_string()));
    }

    // Replace placeholders with actual values
    let html_content = html_template
        .replace("{{directory}}", &directory)
        .replace("{{file_list}}", &file_list_html);

    Ok(Html(html_content))
}

pub fn file_list(directory: &str) -> String {
    let paths = fs::read_dir(directory).unwrap_or_else(|_| {
        panic!("Unable to read directory '{}'", directory);
    });

    let mut file_list_html = String::new();

    for path in paths {
        if let Ok(entry) = path {
            let file_name = entry.file_name().into_string().unwrap();
            let file_type = if entry.file_type().unwrap().is_dir() {
                "directory"
            } else {
                "file"
            };

            // Show button for folders
            if file_type == "directory" {
                file_list_html.push_str(&format!(
                    r#"<li class="{}">
                        <div class="file-entry">
                            <button onclick="location.href='/{}'">{}</button>
                        </div>
                    </li>"#,
                    file_type, format!("{}/{}", directory.strip_prefix("files/").unwrap_or(directory), file_name), file_name
                ));
            } else {
                file_list_html.push_str(&format!(
                    r#"<li class="{}">
                        <div class="file-entry">
                            <span>{}</span>
                            <a href="/download/{}" class="custom-button">Download</a>
                        </div>
                    </li>"#,
                    file_type, file_name, format!("{}/{}", directory.strip_prefix("files/").unwrap_or(directory), file_name)
                ));
            }
        }
    }

    file_list_html
}

pub async fn download_file(Path(path): Path<String>) -> impl IntoResponse {
    let file_path = format!("files/{}", path);

    // Check if the file exists
    if !StdPath::new(&file_path).exists() {
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }

    // Read the file contents
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to open file").into_response(),
    };

    let mut contents = Vec::new();
    if let Err(_) = file.read_to_end(&mut contents) {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file").into_response();
    }

    let file_name = StdPath::new(&file_path).file_name().unwrap().to_string_lossy().to_string();
    let disposition = format!("attachment; filename=\"{}\"", file_name);

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_DISPOSITION, disposition)
        .body(contents.into())
        .unwrap()
}