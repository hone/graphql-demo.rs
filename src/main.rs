use juniper::{EmptyMutation, EmptySubscription};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

mod model;
mod web;
use model::{Ctx, Query, Schema};
use web::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let schema = Arc::new(Schema::new(
        Query {},
        EmptyMutation::new(),
        EmptySubscription::new(),
    ));
    let ctx = Arc::new(Ctx {
        data: toml::from_str(include_str!("../data.toml")).unwrap(),
    });
    let state = AppState { ctx, schema };

    let app = web::routes().with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
