extern crate actix;

use actix::prelude::*;
use juniper::FieldResult;
use futures::Future;

use super::entities::Player;
use super::super::db::{DBExecutor, Messages, Responses};

pub fn get_player(db: Addr<DBExecutor>, user_id: String) -> FieldResult<Option<Player>> {
    println!("{:?}", db);

    db.send(Messages::GetPlayerById(user_id))
        .and_then(|res| {
            println!("Some stuff: {:?}", res);
            Ok(res)
        }).wait();

    Ok(Some(Player {
        id: "123".to_owned(),
        name: "Tom Arrell".to_owned(),
        realm_id: "movio".to_owned(),
        sessions: vec![],
        historical_balance: 100,
        real_balance: 50,
        total_buyin: 200,
    }))
}
