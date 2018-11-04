extern crate actix;

use actix::prelude::*;
use futures::Future;
use juniper::FieldResult;

use db::{DBExecutor, Messages, Responses};
use graphql::entities::Player;

pub fn get_player(db: Addr<DBExecutor>, user_id: i32) -> FieldResult<Option<Player>> {
    let response = db
        .send(Messages::GetPlayerById(user_id))
        .wait()
        .and_then(|res| {
            let db_value = res.expect("DBExecutor failed to execute DB query.");

            match db_value {
                Responses::Player(player) => Ok(player),
            }
        })
        .expect("Failed to receive message from DBExecutor, inbox closed or message timed out.");

    Ok(response)
}
