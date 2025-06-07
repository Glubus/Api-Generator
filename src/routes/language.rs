//! # Help Routes Module
//!
//! Ce module configure les routes d'aide et de diagnostic de l'API.

use axum::{routing::get, Router};
use crate::{db::DatabaseManager, handlers::language};

/// Créer le routeur pour les routes d'aide
pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/language", get(language::get_languages))
        .route("/language/{id}", get(language::get_language_by_id))
        .route("/language/{id}/frameworks/{type}", get(language::get_language_frameworks_by_type))
} 