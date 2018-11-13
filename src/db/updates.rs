use rusqlite::types::ToSql;
use rusqlite::Error;

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
