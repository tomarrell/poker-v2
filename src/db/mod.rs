use actix::prelude::*;
use r2d2;
use r2d2_sqlite;
use rusqlite::{NO_PARAMS, Error};

use super::schema::{Player};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub struct DBExecutor(pub Pool);

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

enum Queries {
    RealmByName,
    RealmByID,
    PlayerByID,
    SessionByID,
    SessionsByRealmID,
}

enum Mutations {
    CreateRealm,
    CreatePlayer,
    PutSession,
}

enum QueryResults {
    Player,
    Realm,
    Session,
}

impl Message for Queries {
    type Result = Result<Vec<QueryResults>, Error>;
}

fn get_users(conn: Connection) -> Result<Vec<Player>, Error> {
    let stmt = "SELECT * FROM player";

    let mut prep_stmt = conn.prepare(stmt)?;
    let users = prep_stmt
        .query_map(NO_PARAMS, |mut row| Player {
            id: row.get(0),
            name: row.get(1),
            realm_id: row.get(2),
            sessions: vec![],
            historical_balance: row.get(3),
            real_balance: row.get(4),
            total_buyin: row.get(5),
        }).unwrap()
        .collect();

    users
}
