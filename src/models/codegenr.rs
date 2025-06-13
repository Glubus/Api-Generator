use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetCodeRequest {
    pub openapi: String,
    pub framework_id: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DownloadResponse {
    pub download_url: String,
    pub size_bytes: u64,
}

