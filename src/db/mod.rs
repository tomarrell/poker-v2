use actix::prelude::*;
use r2d2;
use r2d2_sqlite;
use rusqlite::{Error, NO_PARAMS};

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
    GetPlayerById(String),
}

#[derive(Debug)]
pub enum Responses {
    Player(Option<Player>),
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

        println!("GOT MESSAGE");

        match msg {
            Messages::GetPlayerById(user_id) => get_player_by_id(db, user_id),
        }
    }
}

fn get_player_by_id(conn: Connection, user_id: String) -> Result<Responses, Error> {
    let stmt = "SELECT * FROM player WHERE userId=1";

    let mut prep_stmt = conn.prepare(stmt)?;
    // let user = prep_stmt
    // .query_row(&[&user_id], |row| Player {
    // id: row.get(0),
    // name: row.get(1),
    // realm_id: row.get(2),
    // sessions: vec![],
    // historical_balance: row.get(3),
    // real_balance: row.get(4),
    // total_buyin: row.get(5),
    // });
    let user: Result<Player, Error> = Ok(Player {
        id: "123".to_owned(),
        name: "Tom Arrell".to_owned(),
        realm_id: "movio".to_owned(),
        sessions: vec![],
        historical_balance: 100,
        real_balance: 50,
        total_buyin: 200,
    });

    match user {
        Ok(u) => Ok(Responses::Player(Some(u))),
        Err(e) => Ok(Responses::Player(None)),
    }
}
