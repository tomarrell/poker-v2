extern crate actix;
extern crate juniper;

use juniper::{FieldResult, RootNode};

use super::entities::Player;
use super::resolvers::get_player;
use super::Context;

// Queries
pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field player(&executor, user_id: i32) -> FieldResult<Option<Player>> {
        let db = executor.context().db.clone();
        get_player(db, user_id)
    }
});

// Mutations
pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    field create_player(&executor) -> FieldResult<()> {
        Ok(())
    }
});

// Schema Init
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
