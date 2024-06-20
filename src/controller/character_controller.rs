use axum::{Router, routing::get, Json};
use axum::extract::Path;
use axum::response::IntoResponse;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_characters))
        .route("/:id", get(get_character))
}

async fn get_characters() -> impl IntoResponse {
    Json("List of characters")
}

async fn get_character(Path(id): Path<String>) -> impl IntoResponse {
    Json(format!("Character with id: {}", id))
}
