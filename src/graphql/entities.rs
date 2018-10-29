extern crate juniper;

use juniper::{GraphQLObject};

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
