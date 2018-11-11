extern crate actix;
extern crate juniper;

use juniper::{FieldResult, RootNode};

use super::entities::*;
use db::{Messages, Responses};
use graphql::{query_db, Context};

// Queries
pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field player(&executor, user_id: i32) -> FieldResult<Option<Player>> {
        let result = query_db(executor, Messages::GetPlayerById(user_id));

        match result {
            Ok(Responses::Player(player)) => Ok(player),
            _ => panic!("Actor returned unexpected message"),
        }
    }

    field realm(&executor, realm_id: i32) -> FieldResult<Option<Realm>> {
        let result = query_db(executor, Messages::GetRealmById(realm_id));

        match result {
            Ok(Responses::Realm(realm)) => Ok(realm),
            _ => panic!("Actor returned unexpected message"),
        }
    }

    field session(&executor, session_id: i32) -> FieldResult<Option<Session>> {
        let result = query_db(executor, Messages::GetSessionById(session_id));

        match result {
            Ok(Responses::Session(session)) => Ok(session),
            _ => panic!("Actor returned unexpected message"),
        }
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
