use actix::prelude::*;
use r2d2;
use r2d2_sqlite;
use rusqlite::Error;

use graphql::entities::*;

mod queries;
use self::queries::*;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Debug)]
pub struct DBExecutor(pub Pool);

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug)]
pub enum Messages {
    // Player
    GetPlayerById(i32),
    GetBuyinByPlayerId(i32),
    GetHistoricalBalanceByPlayerId(i32),
    GetRealBalanceByPlayerId(i32),
    GetPlayerSessionsByPlayerId(i32),

    // Realm
    GetRealmById(i32),
    GetPlayersByRealmId(i32),
    GetSessionsByRealmId(i32),

    // Session
    GetSessionById(i32),
    GetPlayerSessionsBySessionId(i32),
}

#[derive(Debug)]
pub enum Responses {
    // Player
    Player(Option<Player>),
    PlayerBalance(i32),
    PlayerSessions(Vec<PlayerSession>),

    // Realm
    Realm(Option<Realm>),
    Players(Vec<Player>),
    Sessions(Vec<Session>),

    // Sessions
    Session(Option<Session>),
}

impl Message for Messages {
    type Result = Result<Responses, Error>;
}

impl Handler<Messages> for DBExecutor {
    type Result = Result<Responses, Error>;

    fn handle(&mut self, msg: Messages, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let db = self
            .0
            .get()
            .expect("Failed to get database connection from pool");

        let res = match msg {
            // Player
            Messages::GetPlayerById(id) => get_player_by_id(db, id),
            Messages::GetBuyinByPlayerId(id) => get_buyin_by_player_id(db, id),
            Messages::GetHistoricalBalanceByPlayerId(id) => {
                get_historical_balance_by_player_id(db, id)
            }
            Messages::GetRealBalanceByPlayerId(id) => get_real_balance_by_player_id(db, id),
            Messages::GetPlayerSessionsByPlayerId(id) => get_player_sessions_by_player_id(db, id),

            // Realm
            Messages::GetRealmById(id) => get_realm_by_id(db, id),
            Messages::GetPlayersByRealmId(id) => get_players_by_realm_id(db, id),
            Messages::GetSessionsByRealmId(id) => get_sessions_by_realm_id(db, id),

            // Session
            Messages::GetSessionById(id) => get_session_by_id(db, id),
            Messages::GetPlayerSessionsBySessionId(id) => get_player_sessions_by_session_id(db, id),
        }
        .expect("DB query failed");

        Ok(res)
    }
}
