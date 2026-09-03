use axum::{routing::{post, get}, Router};
use crate::core::router::{chat_completions, image_gen, health};

pub fn build_router() -> Router {
    Router::new()
       .route("/v1/chat/completions", post(chat_completions))
       .route("/v1/images/generations", post(image_gen))
       .route("/health", get(health))
       .route("/v1/books/generate", post(crate::book::factory::api_generate))
}
