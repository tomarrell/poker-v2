use graphql::entities::*;
use rusqlite::Error;

use super::{Connection, Responses};

pub fn get_player_by_id(conn: Connection, user_id: i32) -> Result<Responses, Error> {
    let stmt = "
        SELECT id, name, realm_id, utc_created_at
        FROM player
        WHERE id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let user = prep_stmt.query_row(&[&user_id], |row| Player {
        id: row.get(0),
        name: row.get(1),
        realm_id: row.get(2),
        utc_created_at: row.get(3),
    });

    match user {
        Ok(u) => Ok(Responses::Player(Some(u))),
        Err(_) => Ok(Responses::Player(None)),
    }
}

pub fn get_buyin_by_player_id(conn: Connection, user_id: i32) -> Result<Responses, Error> {
    let stmt = "
        SELECT COALESCE(SUM(buyin), 0)
        FROM player_session
        WHERE player_id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let user: Result<i32, Error> = prep_stmt.query_row(&[&user_id], |row| row.get(0));

    match user {
        Ok(u) => Ok(Responses::PlayerBalance(u)),
        Err(_) => Ok(Responses::PlayerBalance(0)),
    }
}

pub fn get_historical_balance_by_player_id(
    conn: Connection,
    user_id: i32,
) -> Result<Responses, Error> {
    let stmt = "
        SELECT COALESCE(SUM(walkout) - SUM(buyin), 0)
        FROM player_session
        WHERE player_id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let user: Result<i32, Error> = prep_stmt.query_row(&[&user_id], |row| row.get(0));

    match user {
        Ok(u) => Ok(Responses::PlayerBalance(u)),
        Err(_) => Ok(Responses::PlayerBalance(0)),
    }
}

pub fn get_real_balance_by_player_id(conn: Connection, user_id: i32) -> Result<Responses, Error> {
    let stmt = "
        SELECT COALESCE(SUM(amount), 0)
        FROM transfer
        WHERE player_id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let user: Result<i32, Error> = prep_stmt.query_row(&[&user_id], |row| row.get(0));

    match user {
        Ok(u) => Ok(Responses::PlayerBalance(u)),
        Err(_) => Ok(Responses::PlayerBalance(0)),
    }
}

pub fn get_player_sessions_by_player_id(conn: Connection, user_id: i32) -> Result<Responses, Error> {
    let stmt = "
        SELECT player_id, session_id, buyin, walkout, utc_created_at
        FROM player_session 
        WHERE player_id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let user: Result<Vec<PlayerSession>, Error> = prep_stmt.query_map(&[&user_id], |row| {
        PlayerSession {
            player_id: row.get(0),
            session_id: row.get(1),
            buyin: row.get(2),
            walkout: row.get(3),
            utc_created_at: row.get(4),
        }
    }).unwrap().collect();

    match user {
        Ok(u) => Ok(Responses::PlayerSessions(u)),
        Err(_) => Ok(Responses::PlayerSessions(vec![])),
    }
}
