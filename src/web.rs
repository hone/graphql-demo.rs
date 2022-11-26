use axum::{
    routing::{get, post},
    Router,
};

mod handlers;

/// All Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/graphiql", get(handlers::graphiql))
        .route("/graphql", post(handlers::graphql))
}
