use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    db::DatabaseManager,
    models::language::{FrameworkWithLanguage, ProgrammingLanguage},
};

/// Get all frameworks
#[utoipa::path(
    get,
    path = "/api/framework",
    responses(
        (status = 200, description = "List of frameworks", body = Vec<FrameworkWithLanguage>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Frameworks"
)]
pub async fn get_frameworks(
    State(db): State<DatabaseManager>,
) -> Result<Json<Vec<FrameworkWithLanguage>>, StatusCode> {
    let frameworks = sqlx::query!(
        r#"
        SELECT 
            f.id, f.name, f.f_type::text as f_type, f.created_at, f.updated_at,
            l.id as language_id, l.name as language_name, 
            l.created_at as "language_created_at!", l.updated_at as "language_updated_at!"
        FROM frameworks f
        JOIN programming_languages l ON f.language_id = l.id
        ORDER BY f.name
        "#
    )
    .fetch_all(db.get_pool())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|row| FrameworkWithLanguage {
        id: row.id,
        name: row.name,
        f_type: row.f_type,
        language: ProgrammingLanguage {
            id: row.language_id,
            name: row.language_name,
            created_at: row.language_created_at,
            updated_at: row.language_updated_at,
        },
        created_at: row.created_at,
        updated_at: row.updated_at,
    })
    .collect();

    Ok(Json(frameworks))
}

/// Get a framework by ID with its language
#[utoipa::path(
    get,
    path = "/api/framework/{id}",
    responses(
        (status = 200, description = "Framework with its language", body = FrameworkWithLanguage),
        (status = 404, description = "Framework not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Framework ID")
    ),
    tag = "Frameworks"
)]
pub async fn get_framework_by_id(
    State(db): State<DatabaseManager>,
    Path(id): Path<i32>,
) -> Result<Json<FrameworkWithLanguage>, StatusCode> {
    let framework = sqlx::query!(
        r#"
        SELECT 
            f.id, f.name, f.f_type::text as f_type, f.created_at, f.updated_at,
            l.id as language_id, l.name as language_name, 
            l.created_at as "language_created_at!", l.updated_at as "language_updated_at!"
        FROM frameworks f
        JOIN programming_languages l ON f.language_id = l.id
        WHERE f.id = $1
        "#,
        id
    )
    .fetch_optional(db.get_pool())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(FrameworkWithLanguage {
        id: framework.id,
        name: framework.name,
        f_type: framework.f_type,
        language: ProgrammingLanguage {
            id: framework.language_id,
            name: framework.language_name,
            created_at: framework.language_created_at,
            updated_at: framework.language_updated_at,
        },
        created_at: framework.created_at,
        updated_at: framework.updated_at,
    }))
} 