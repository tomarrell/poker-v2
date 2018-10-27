extern crate juniper;

use juniper::{FieldResult, GraphQLInputObject, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub realm_id: String,
    pub sessions: Vec<PlayerSession>,
    pub historical_balance: i32,
    pub real_balance: i32,
    pub total_buyin: i32,
}

#[derive(GraphQLObject)]
pub struct Realm {
    pub id: String,
    pub name: String,
    pub title: String,
    pub players: Vec<Player>,
    pub sessions: Vec<Session>,
}

#[derive(GraphQLObject)]
pub struct Session {
    pub id: String,
    pub realm_id: String,
    pub name: String,
    pub time: String,
    pub player_sessions: Vec<PlayerSession>,
}

#[derive(GraphQLObject)]
pub struct PlayerSession {
    pub player: Player,
    pub player_id: String,
    pub sessions_id: String,
    pub buyin: i32,
    pub walkout: i32,
}

// Queries
pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field person(&executor, id: String) -> FieldResult<Player> {
        Ok(Player {
             id: "1234".to_owned(),
             name: String::from("Tom Arrell"),
             realm_id: String::from("1"),
             sessions: vec![],
             historical_balance: 100,
             real_balance: 100,
             total_buyin: 50,
        })
    }
});

// Mutations
pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field null
});

// Schema Init
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
