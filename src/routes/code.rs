use axum::{routing::post, Router};
use crate::{db::DatabaseManager, handlers::exec};

pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/code", post(exec::get_code))
}