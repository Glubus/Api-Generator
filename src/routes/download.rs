use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
    extract::Path,
};
use std::path::Path as StdPath;
use std::fs;
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE, HeaderValue};
use crate::db::DatabaseManager;
pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/download/{filename}", get(download_file))
}

async fn download_file(Path(filename): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let file_path = StdPath::new("./public").join(&filename);
    
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let content = fs::read(&file_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content_type = HeaderValue::from_static("application/zip");
    let content_disposition = HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let headers = [
        (CONTENT_TYPE, content_type),
        (CONTENT_DISPOSITION, content_disposition),
    ];

    Ok((headers, content))
} 