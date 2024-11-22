use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use std::fs;

pub(crate) async fn serve_index() -> impl IntoResponse {
    match fs::read("_site/index.html") {
        Ok(contents) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(Body::from(contents))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("`index.html` not found"))
            .unwrap(),
    }
}

pub(crate) async fn serve_static(Path(path): Path<String>) -> impl IntoResponse {
    let file_path = match path.starts_with("styles/") {
        true => format!("styles/{}", &path[6..]),
        false => format!("_site/{}", path),
    };

    let content_type = match path {
        path if path.ends_with(".html") => "text/html",
        path if path.ends_with(".css") => "text/css",
        path if path.ends_with(".png") => "image/png",
        path if path.ends_with(".jpg") || path.ends_with(".jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    };

    match fs::read(&file_path) {
        Ok(contents) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", content_type)
            .body(Body::from(contents))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap(),
    }
}
