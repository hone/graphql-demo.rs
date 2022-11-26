use crate::model::{Ctx, Schema};
use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

mod handlers;

#[derive(Clone, FromRef)]
/// Axum State
pub struct AppState {
    pub schema: Arc<Schema>,
    pub ctx: Arc<Ctx>,
}

/// All Routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::root))
        .route("/graphiql", get(handlers::graphiql))
        .route("/graphql", post(handlers::graphql))
}
