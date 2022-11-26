use axum::{
    routing::{get, post},
    Router,
};

mod handlers;

/// All Routes
pub fn routes() -> Router<axum::body::Body> {
    Router::new()
        .route("/", get(handlers::root))
        .route("/graphiql", get(handlers::graphiql))
        .route("/graphql", post(handlers::graphql))
}
