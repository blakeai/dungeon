use axum::{Router, routing::get, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::log::error;
use crate::service::class_loader::extract_classes;

pub fn router() -> Router {
    Router::new()
        .route("/all", get(get_characters))
}

async fn get_characters() -> impl IntoResponse {
    match extract_classes() {
        Ok(classes) => Json(classes).into_response(),
        Err(e) => {
            error!("Failed to extract classes: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to load character classes"})),
            ).into_response()
        }
    }
}