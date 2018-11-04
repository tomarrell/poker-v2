use actix::prelude::*;
use r2d2;
use r2d2_sqlite;
use rusqlite::Error;

use graphql::entities::Player;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Debug)]
pub struct DBExecutor(pub Pool);

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug)]
pub enum Messages {
    GetPlayerById(i32),
    GetBuyinByPlayerId(i32),
}

#[derive(Debug)]
pub enum Responses {
    Player(Option<Player>),
    PlayerBuyin(i32),
}

impl Message for Messages {
    type Result = Result<Responses, Error>;
}

impl Handler<Messages> for DBExecutor {
    type Result = Result<Responses, Error>;

    fn handle(&mut self, msg: Messages, ctx: &mut SyncContext<Self>) -> Self::Result {
        let db = self
            .0
            .get()
            .expect("Failed to get database connection from pool");

        let res = match msg {
            Messages::GetPlayerById(id) => get_player_by_id(db, id),
            Messages::GetBuyinByPlayerId(id) => get_buyin_by_player_id(db, id),
        }
        .expect("DB query failed");

        Ok(res)
    }
}

fn get_player_by_id(conn: Connection, user_id: i32) -> Result<Responses, Error> {
    let stmt = "SELECT * FROM player WHERE id=?";

    let mut prep_stmt = conn.prepare(stmt)?;
    let user = prep_stmt.query_row(&[&user_id], |row| {
        Player {
            id: row.get(0),
            name: row.get(1),
            realm_id: row.get(2),
            utc_created_at: row.get(3),
        }
    });

    match user {
        Ok(u) => Ok(Responses::Player(Some(u))),
        Err(e) => Ok(Responses::Player(None)),
    }
}

fn get_buyin_by_player_id(conn: Connection, user_id: i32) -> Result<Responses, Error> {
    let stmt = "SELECT * FROM player WHERE id=?";

    let mut prep_stmt = conn.prepare(stmt)?;
    let user = prep_stmt.query_row(&[&user_id], |row| {
        Player {
            id: row.get(0),
            name: row.get(1),
            realm_id: row.get(2),
            utc_created_at: row.get(3),
        }
    });

    match user {
        Ok(u) => Ok(Responses::Player(Some(u))),
        Err(e) => Ok(Responses::Player(None)),
    }
}
