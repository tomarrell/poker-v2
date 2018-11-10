extern crate juniper;

use futures::Future;

use db::{Messages, Responses};
use graphql::Context;

#[derive(Debug)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub realm_id: i32,
    pub utc_created_at: String,
}

graphql_object!(Player: Context |&self| {
    description: "A player of a game"

    field id() -> i32 as "The unique id of a Player" {
        self.id
    }

    field name() -> &str as "The name of a Player" {
        &self.name
    }

    field realm_id() -> i32 as "The id of the Realm the Player is within" {
        self.realm_id
    }

    field sessions() -> Vec<PlayerSession> as "The sessions the Player has participated in" {
        unimplemented!()
    }

    field historical_balance() -> i32 as "The amount of money a Player has won or lost in total, does not include rebalances" {
        unimplemented!()
    }

    field real_balance() -> i32 as "The amount of money a Player has won or lost in addition to any rebalances the player has made" {
        unimplemented!()
    }

    field total_buyin(&executor) -> i32 as "The total amount of money the Player has bought in with" {
        let result = executor
            .context()
            .db
            .send(Messages::GetBuyinByPlayerId(self.id))
            .wait()
            .and_then(|res| {
                let db_value = res.expect("DBExecutor failed to execute DB query");
                match db_value {
                    Responses::PlayerBuyin(amount) => Ok(amount),
                    _ => panic!("Actor returned unexpected message"),
                }
            })
            .expect("Failed to receive message from DBExecutor, inbox closed or message timed out.");

        result
    }
});

#[derive(Debug)]
pub struct Realm {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub utc_created_at: String,
}

graphql_object!(Realm: Context |&self| {
    description: "A 'world' for a specific recurring series of games to be played"

    field id() -> i32 as "The id of the Realm" {
        self.id
    }

    field name() -> &str as "The name of the Realm" {
        &self.name
    }

    field title() -> &str as "A user changeable title for the Realm" {
        &self.title
    }

    field players() -> Vec<Player> as "A list of all the Players in the Realm" {
        unimplemented!()
    }

    field sessions() -> Vec<Session> as "A list of all the Sessions played within the Realm" {
        unimplemented!()
    }
});

#[derive(Debug)]
pub struct Session {
    pub id: i32,
    pub realm_id: i32,
    pub name: String,
    pub time: String,
    pub utc_created_at: String,
}

graphql_object!(Session: Context |&self| {
    description: "A game single instance of a game being played"

    field id() -> i32 as "The id of the Session which was played" {
        self.id
    }

    field realm_id() -> i32 as "The realm id which the Session was played under" {
        self.realm_id
    }

    field name() -> &str as "The name of the Session" {
        &self.name
    }

    field time() -> &str as "The time the session occurred" {
        &self.time
    }

    field player_sessions() -> Vec<PlayerSession> as "The list of Players who participated in this Session" {
        unimplemented!()
    }
});

#[derive(Debug)]
pub struct PlayerSession {
    pub player_id: i32,
    pub session_id: i32,
    pub buyin: i32,
    pub walkout: i32,
    pub utc_created_at: String,
}

graphql_object!(PlayerSession: Context |&self| {
    description: "A participation by a Player in a Session"

    field player() -> Player as "The Player who participated in the Session" {
        unimplemented!()
    }

    field player_id() -> i32 as "The id of a Player who participated in a Session" {
        self.player_id
    }

    field session_id() -> i32 as "The id of the Session which was participated in" {
        self.session_id
    }

    field buyin() -> i32 as "The amount of money the Player bought in with" {
        self.buyin
    }

    field buyin() -> i32 as "The amount of money the Player walked out with" {
        self.walkout
    }
});
