use crate::model::{Ctx, Schema};

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use juniper::http::GraphQLRequest;
use serde_json::json;
use std::sync::Arc;
use tracing::info;

pub async fn root() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message": "Hello, World!"})))
}

pub async fn graphiql() -> impl IntoResponse {
    Html(juniper::http::graphiql::graphiql_source("/graphql", None))
}

pub async fn graphql(
    State(schema): State<Arc<Schema>>,
    State(ctx): State<Arc<Ctx>>,
    req: Json<GraphQLRequest>,
) -> impl IntoResponse {
    info!("{:#?}", req);
    let response = req.execute(&schema, &ctx).await;
    let status = if response.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    let json = serde_json::to_string(&response).unwrap();

    (status, json)
}
