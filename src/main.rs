use axum::Extension;
use juniper::{EmptyMutation, EmptySubscription};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

mod model;
mod web;
use crate::model::{Ctx, Query, Schema};

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

    let app = web::routes().layer(Extension(schema)).layer(Extension(ctx));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
