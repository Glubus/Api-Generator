use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ProgrammingLanguage {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FrameworkType {
    Client,
    Server,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Framework {
    pub id: i32,
    pub name: String,
    pub language_id: i32,
    pub f_type: Option<String>,
    pub codegenr_config: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// DTOs for responses that include related data
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FrameworkWithLanguage {
    pub id: i32,
    pub name: String,
    pub f_type: Option<String>,
    pub codegenr_config: Option<Vec<String>>,
    pub language: ProgrammingLanguage,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LanguageWithFrameworks {
    pub id: i32,
    pub name: String,
    pub frameworks: Vec<Framework>,
}
