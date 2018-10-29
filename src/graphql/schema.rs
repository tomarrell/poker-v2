extern crate juniper;

use juniper::{FieldResult, RootNode};

use super::entities::Player;
use super::resolvers::get_player;

// Queries
pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field player(user_id: String) -> FieldResult<Option<Player>> {
        get_player(user_id)
    }
});

// Mutations
pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field create_player(&executor) -> FieldResult<()> {
        Ok(())
    }
});

// Schema Init
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
