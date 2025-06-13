//! # Routes Module
//!
//! Ce module gère la configuration des routes de l'API.
//! Il permet d'organiser les routes par domaine fonctionnel et de les combiner
//! dans un routeur Axum unique.
//!
//! ## Utilisation
//!
//! Pour ajouter de nouvelles routes :
//! 1. Créez un nouveau module dans le dossier `routes/`
//! 2. Implémentez une fonction `router()` qui retourne un `Router`
//! 3. Ajoutez le module dans ce fichier
//! 4. Utilisez `merge()` pour combiner les routes

use crate::db::DatabaseManager;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

// Re-export all route modules here
pub mod help;
pub mod language;
pub mod framework;
pub mod code;
pub mod download;

#[derive(OpenApi)]
#[openapi(paths(crate::handlers::help::health_check, crate::handlers::help::health_light,
     crate::handlers::help::info, crate::handlers::help::ping,
     crate::handlers::language::get_languages, crate::handlers::language::get_language_by_id,
     crate::handlers::framework::get_frameworks, crate::handlers::framework::get_framework_by_id))]
struct ApiDoc;

pub fn create_router(db: DatabaseManager) -> Router {
    Router::new()
        .nest("/api", help::router())
        .nest("/api", language::router())
        .nest("/api", framework::router())
        .nest("/api", code::router())
        .nest("/api", download::router())
        .merge(SwaggerUi::new("/api/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(db)
}
