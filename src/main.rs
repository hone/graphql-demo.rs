use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use juniper::{
    graphql_object, http::GraphQLRequest, EmptyMutation, EmptySubscription, FieldResult,
    GraphQLObject, RootNode,
};
use serde::Deserialize;
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};

#[derive(Clone, Deserialize, GraphQLObject)]
#[graphql(description = "A link")]
struct Link {
    id: i32,
    url: String,
    description: String,
}

#[derive(Clone, Deserialize)]
struct Data {
    links: Vec<Link>,
}

#[derive(Clone)]
struct Ctx {
    pub data: Data,
}

impl juniper::Context for Ctx {}

struct Query;

#[graphql_object(Context = Ctx)]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }

    fn all_links(context: &Ctx) -> FieldResult<&Vec<Link>> {
        Ok(&context.data.links)
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;

#[tokio::main]
async fn main() {
    let schema = Arc::new(Schema::new(
        Query {},
        EmptyMutation::new(),
        EmptySubscription::new(),
    ));

    let ctx = Arc::new(Ctx {
        data: toml::from_str(include_str!("../data.toml")).unwrap(),
    });

    let req = GraphQLRequest::<juniper::DefaultScalarValue>::new(
        String::from("query { allLinks { url } }"),
        None,
        None,
    );
    let resp = req.execute(&schema, &ctx).await;
    println!("{:?}", resp);

    let app = Router::new()
        .route("/", get(root))
        .route("/graphiql", get(graphiql))
        .route("/graphql", post(graphql))
        .layer(Extension(schema))
        .layer(Extension(ctx));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"message": "Hello, World!"})))
}

async fn graphiql() -> impl IntoResponse {
    Html(juniper::http::graphiql::graphiql_source("/graphql", None))
}

async fn graphql(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(ctx): Extension<Arc<Ctx>>,
    req: Json<GraphQLRequest>,
) -> impl IntoResponse {
    let response = req.execute(&schema, &ctx).await;
    let status = if response.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    let json = serde_json::to_string(&response).unwrap();

    (status, json)
}
