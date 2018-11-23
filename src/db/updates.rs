use rusqlite::types::ToSql;
use rusqlite::Error;

use crate::graphql::input_types::InputPlayerSession;

use super::{Connection, Responses};

pub fn create_realm(
    conn: Connection,
    name: String,
    title: Option<String>,
) -> Result<Responses, Error> {
    let stmt = "
        INSERT INTO realm(name, title)
        VALUES(?1, ?2)
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    prep_stmt.insert(&[&name, &title as &ToSql])?;

    Ok(Responses::Ok)
}

pub fn create_player(conn: Connection, name: String, realm_id: i32) -> Result<Responses, Error> {
    let stmt = "
        INSERT INTO player(name, realm_id)
        VALUES(?1, ?2)
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    prep_stmt.insert(&[&name, &realm_id as &ToSql])?;

    Ok(Responses::Ok)
}

pub fn create_session(
    mut conn: Connection,
    name: String,
    realm_id: i32,
    time: String,
    player_sessions: Vec<InputPlayerSession>,
) -> Result<Responses, Error> {
    let tx = conn.transaction()?;

    let stmt = "
        INSERT INTO session(name, realm_id, utc_time)
        VALUES (?1, ?2, ?3)
    ";

    // Insert the session into the `session` table
    tx.prepare(stmt)?
        .insert(&[&name, &realm_id as &ToSql, &time])?;

    // Fetch most recent session insertion
    let session_id = tx.last_insert_rowid();

    let stmt = "
        INSERT INTO player_session(player_id, session_id, buyin, walkout)
        VALUES (?1, ?2, ?3, ?4)
    ";

    // Iterate over player_sessions and insert a row into
    // `player_session` table for each
    {
        let mut prep_stmt = tx.prepare(stmt)?;
        player_sessions.iter().for_each(|ps| {
            let _ =
                prep_stmt.insert(&[&ps.player_id, &session_id as &ToSql, &ps.buyin, &ps.walkout]);
        });
    }

    tx.commit()?;

    Ok(Responses::Ok)
}

pub fn modify_session(
    conn: Connection,
    id: i32,
    name: String,
    realm_id: i32,
    time: String,
    player_sessions: Vec<InputPlayerSession>,
) -> Result<Responses, Error> {
    unimplemented!()
}
