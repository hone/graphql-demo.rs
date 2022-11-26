use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode,
};
use serde::Deserialize;

#[derive(Clone, Deserialize, GraphQLObject)]
#[graphql(description = "A link")]
/// HackerNews Link
struct Link {
    id: i32,
    url: String,
    description: String,
}

#[derive(Clone, Deserialize)]
/// TOML data document
pub struct Data {
    links: Vec<Link>,
}

#[derive(Clone)]
/// Context struct to hold data
pub struct Ctx {
    pub data: Data,
}

impl juniper::Context for Ctx {}

/// GraphQL Root Query
pub struct Query;

#[graphql_object(Context = Ctx)]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }

    fn all_links(context: &Ctx) -> FieldResult<&Vec<Link>> {
        Ok(&context.data.links)
    }
}

/// GraphQL RootNode
pub type Schema = RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;
