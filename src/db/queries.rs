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
    let player = prep_stmt.query_row(&[&user_id], |row| Player {
        id: row.get(0),
        name: row.get(1),
        realm_id: row.get(2),
        utc_created_at: row.get(3),
    });

    match player {
        Ok(p) => Ok(Responses::Player(Some(p))),
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
    let buyin: Result<i32, Error> = prep_stmt.query_row(&[&user_id], |row| row.get(0));

    match buyin {
        Ok(b) => Ok(Responses::PlayerBalance(b)),
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
    let balance: Result<i32, Error> = prep_stmt.query_row(&[&user_id], |row| row.get(0));

    match balance {
        Ok(b) => Ok(Responses::PlayerBalance(b)),
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
    let balance: Result<i32, Error> = prep_stmt.query_row(&[&user_id], |row| row.get(0));

    match balance {
        Ok(b) => Ok(Responses::PlayerBalance(b)),
        Err(_) => Ok(Responses::PlayerBalance(0)),
    }
}

pub fn get_player_sessions_by_player_id(
    conn: Connection,
    user_id: i32,
) -> Result<Responses, Error> {
    let stmt = "
        SELECT player_id, session_id, buyin, walkout, utc_created_at
        FROM player_session
        WHERE player_id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let player_sessions: Result<Vec<PlayerSession>, Error> = prep_stmt
        .query_map(&[&user_id], |row| PlayerSession {
            player_id: row.get(0),
            session_id: row.get(1),
            buyin: row.get(2),
            walkout: row.get(3),
            utc_created_at: row.get(4),
        })
        .unwrap()
        .collect();

    match player_sessions {
        Ok(ps) => Ok(Responses::PlayerSessions(ps)),
        Err(_) => Ok(Responses::PlayerSessions(vec![])),
    }
}

pub fn get_realm_by_id(conn: Connection, realm_id: i32) -> Result<Responses, Error> {
    let stmt = "
        SELECT id, name, title, utc_created_at
        FROM realm
        WHERE id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let realm: Result<Option<Realm>, Error> = prep_stmt.query_row(&[&realm_id], |row| {
        Some(Realm {
            id: row.get(0),
            name: row.get(1),
            title: row.get(2),
            utc_created_at: row.get(3),
        })
    });

    match realm {
        Ok(r) => Ok(Responses::Realm(r)),
        Err(_) => Ok(Responses::Realm(None)),
    }
}

pub fn get_players_by_realm_id(conn: Connection, realm_id: i32) -> Result<Responses, Error> {
    let stmt = "
        SELECT id, name, realm_id, utc_created_at
        FROM player
        WHERE realm_id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let players: Result<Vec<Player>, Error> = prep_stmt
        .query_map(&[&realm_id], |row| Player {
            id: row.get(0),
            name: row.get(1),
            realm_id: row.get(2),
            utc_created_at: row.get(3),
        })
        .unwrap()
        .collect();

    match players {
        Ok(p) => Ok(Responses::Players(p)),
        Err(_) => Ok(Responses::Players(vec![])),
    }
}

pub fn get_sessions_by_realm_id(conn: Connection, realm_id: i32) -> Result<Responses, Error> {
    let stmt = "
        SELECT id, name, realm_id, utc_time, utc_created_at
        FROM session
        WHERE realm_id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let sessions: Result<Vec<Session>, Error> = prep_stmt
        .query_map(&[&realm_id], |row| Session {
            id: row.get(0),
            name: row.get(1),
            realm_id: row.get(2),
            utc_time: row.get(3),
            utc_created_at: row.get(4),
        })
        .unwrap()
        .collect();

    match sessions {
        Ok(s) => Ok(Responses::Sessions(s)),
        Err(_) => Ok(Responses::Sessions(vec![])),
    }
}

pub fn get_session_by_id(conn: Connection, session_id: i32) -> Result<Responses, Error> {
    let stmt = "
        SELECT id, name, realm_id, utc_time, utc_created_at
        FROM session
        WHERE id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let session: Result<Option<Session>, Error> = prep_stmt.query_row(&[&session_id], |row| {
        Some(Session {
            id: row.get(0),
            name: row.get(1),
            realm_id: row.get(2),
            utc_time: row.get(3),
            utc_created_at: row.get(4),
        })
    });

    match session {
        Ok(s) => Ok(Responses::Session(s)),
        Err(_) => Ok(Responses::Session(None)),
    }
}

pub fn get_player_sessions_by_session_id(
    conn: Connection,
    session_id: i32,
) -> Result<Responses, Error> {
    let stmt = "
        SELECT player_id, session_id, buyin, walkout, utc_created_at
        FROM player_session
        WHERE session_id=?
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    let player_sessions: Result<Vec<PlayerSession>, Error> = prep_stmt
        .query_map(&[&session_id], |row| PlayerSession {
            player_id: row.get(0),
            session_id: row.get(1),
            buyin: row.get(2),
            walkout: row.get(3),
            utc_created_at: row.get(4),
        })
        .unwrap()
        .collect();

    match player_sessions {
        Ok(ps) => Ok(Responses::PlayerSessions(ps)),
        Err(_) => Ok(Responses::PlayerSessions(vec![])),
    }
}
