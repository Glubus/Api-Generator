//! # Help Routes Module
//!
//! Ce module configure les routes d'aide et de diagnostic de l'API.

use axum::{routing::get, Router};
use crate::{db::DatabaseManager, handlers::framework};

/// Créer le routeur pour les routes d'aide
pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/framework", get(framework::get_frameworks))
        .route("/framework/{id}", get(framework::get_framework_by_id))
} 