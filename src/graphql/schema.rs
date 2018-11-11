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
        let result = query_db(executor, Messages::GetPlayerById(user_id))?;

        match result {
            Responses::Player(player) => Ok(player),
            _ => Err("Actor returned unexpected message")?,
        }
    }

    field realm(&executor, realm_id: Option<i32>, realm_name: Option<String>) -> FieldResult<Option<Realm>> {
        if let Some(realm_id) = realm_id {
            let result = query_db(executor, Messages::GetRealmById(realm_id))?;

            match result {
                Responses::Realm(realm) => return Ok(realm),
                _ => Err("Actor returned unexpected message")?,
            }
        }

        if let Some(realm_name) = realm_name {
            unimplemented!()
        }

        Err("At least one arg of `realm_id` or `realm_name` must be specified on this query")?
    }

    field session(&executor, session_id: i32) -> FieldResult<Option<Session>> {
        let result = query_db(executor, Messages::GetSessionById(session_id))?;

        match result {
            Responses::Session(session) => Ok(session),
            _ => Err("Actor returned unexpected message")?,
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
