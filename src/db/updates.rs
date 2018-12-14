use rusqlite::types::ToSql;
use rusqlite::Error;

use crate::graphql::input_types::InputPlayerSession;

use super::{Connection, Responses};

pub fn create_realm(
    conn: &Connection,
    name: &str,
    title: &Option<String>,
) -> Result<Responses, Error> {
    let stmt = "
        INSERT INTO realm(name, title)
        VALUES(?1, ?2)
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    prep_stmt.insert(&[&name, &title as &dyn ToSql])?;

    Ok(Responses::Ok)
}

pub fn create_player(conn: &Connection, name: &str, realm_id: i32) -> Result<Responses, Error> {
    let stmt = "
        INSERT INTO player(name, realm_id)
        VALUES(?1, ?2)
    ";

    let mut prep_stmt = conn.prepare(stmt)?;
    prep_stmt.insert(&[&name, &realm_id as &dyn ToSql])?;

    Ok(Responses::Ok)
}

pub fn create_session(
    mut conn: Connection,
    name: &str,
    realm_id: i32,
    time: &str,
    player_sessions: &[InputPlayerSession],
) -> Result<Responses, Error> {
    let tx = conn.transaction()?;

    let stmt = "
        INSERT INTO session(name, realm_id, utc_time)
        VALUES (?1, ?2, ?3)
    ";

    // Insert the session into the `session` table
    tx.prepare(stmt)?
        .insert(&[&name, &realm_id as &dyn ToSql, &time])?;

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
            let _ = prep_stmt.insert(&[
                &ps.player_id,
                &session_id as &dyn ToSql,
                &ps.buyin,
                &ps.walkout,
            ]);
        });
    }

    tx.commit()?;

    Ok(Responses::Ok)
}

pub fn modify_session(
    mut conn: Connection,
    id: i32,
    name: &str,
    time: &str,
    player_sessions: &[InputPlayerSession],
) -> Result<Responses, Error> {
    let tx = conn.transaction()?;

    let update = "
        UPDATE session
        SET name = ?1, utc_time = ?2
        WHERE id = ?3
    ";

    // Update the session record with new name and time
    tx.prepare(update)?
        .execute(&[&name, &time as &dyn ToSql, &id])?;

    // Delete all the old player_sessions
    // TODO: In future, event sourcing?
    let deletion = "
        DELETE FROM player_session WHERE session_id = ?1
    ";

    tx.prepare(deletion)?
        .execute(&[&id])?;

    // Iterate through new player_sessions and insert
    // them into table
    let stmt = "
        INSERT INTO player_session(player_id, session_id, buyin, walkout)
        VALUES (?1, ?2, ?3, ?4)
    ";

    // Iterate over player_sessions and insert a row into
    // `player_session` table for each
    {
        let mut prep_stmt = tx.prepare(stmt)?;
        player_sessions.iter().for_each(|ps| {
            let _ = prep_stmt.insert(&[
                &ps.player_id,
                &id as &dyn ToSql,
                &ps.buyin,
                &ps.walkout,
            ]);
        });
    }

    tx.commit()?;

    Ok(Responses::Ok)
}
