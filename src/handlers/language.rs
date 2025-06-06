use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::{
    db::DatabaseManager,
    models::language::{Framework, LanguageWithFrameworks, ProgrammingLanguage},
};

/// Get all programming languages
#[utoipa::path(
    get,
    path = "/api/language",
    responses(
        (status = 200, description = "List of programming languages", body = Vec<ProgrammingLanguage>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Languages"
)]
pub async fn get_languages(
    State(db): State<DatabaseManager>,
) -> Result<Json<Vec<ProgrammingLanguage>>, StatusCode> {
    let languages = sqlx::query_as!(
        ProgrammingLanguage,
        r#"
        SELECT id, name, created_at as "created_at!", updated_at as "updated_at!"
        FROM programming_languages
        ORDER BY name
        "#
    )
    .fetch_all(db.get_pool())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(languages))
}

/// Get a programming language by ID with its frameworks
#[utoipa::path(
    get,
    path = "/api/language/{id}",
    responses(
        (status = 200, description = "Programming language with its frameworks", body = LanguageWithFrameworks),
        (status = 404, description = "Language not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Language ID")
    ),
    tag = "Languages"
)]
pub async fn get_language_by_id(
    State(db): State<DatabaseManager>,
    Path(id): Path<i32>,
) -> Result<Json<LanguageWithFrameworks>, StatusCode> {
    let language = sqlx::query_as!(
        ProgrammingLanguage,
        r#"
        SELECT id, name, created_at as "created_at!", updated_at as "updated_at!"
        FROM programming_languages
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db.get_pool())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let frameworks = sqlx::query_as!(
        Framework,
        r#"
        SELECT id, name, language_id, f_type::text as f_type, created_at, updated_at
        FROM frameworks
        WHERE language_id = $1
        ORDER BY name
        "#,
        id
    )
    .fetch_all(db.get_pool())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LanguageWithFrameworks {
        id: language.id,
        name: language.name,
        frameworks,
    }))
} 