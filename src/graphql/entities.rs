extern crate juniper;

use super::query_db;

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

    field utc_created_at() -> &str as "The date the PlayerSession was created" {
        &self.utc_created_at
    }

    field player_sessions(&executor) -> Vec<PlayerSession> as "The sessions the Player has participated in" {
        let result = query_db(executor, Messages::GetPlayerSessionsByPlayerId(self.id));

        match result {
            Ok(Responses::PlayerSessions(player_sessions)) => player_sessions,
            _ => panic!("Actor returned unexpected message"),
        }
    }

    field historical_balance(&executor) -> i32 as "The amount of money a Player has won or lost in total, does not include rebalances" {
        let result = query_db(executor, Messages::GetHistoricalBalanceByPlayerId(self.id));

        match result {
            Ok(Responses::PlayerBalance(amount)) => amount,
            _ => panic!("Actor returned unexpected message"),
        }
    }

    field real_balance(&executor) -> i32 as "The amount of money a Player has won or lost in addition to any rebalances the player has made" {
        let result = query_db(executor, Messages::GetRealBalanceByPlayerId(self.id));

        match result {
            Ok(Responses::PlayerBalance(amount)) => amount,
            _ => panic!("Actor returned unexpected message"),
        }
    }

    field total_buyin(&executor) -> i32 as "The total amount of money the Player has bought in with" {
        let result = query_db(executor, Messages::GetBuyinByPlayerId(self.id));

        match result {
            Ok(Responses::PlayerBalance(amount)) => amount,
            _ => panic!("Actor returned unexpected message"),
        }
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

    field utc_created_at() -> &str as "The date the PlayerSession was created" {
        &self.utc_created_at
    }

    field players(&executor) -> Vec<Player> as "A list of all the Players in the Realm" {
        let result = query_db(executor, Messages::GetPlayersByRealmId(self.id));

        match result {
            Ok(Responses::Players(players)) => players,
            _ => panic!("Actor returned unexpected message"),
        }
    }

    field sessions(&executor) -> Vec<Session> as "A list of all the Sessions played within the Realm" {
        let result = query_db(executor, Messages::GetSessionsByRealmId(self.id));

        match result {
            Ok(Responses::Sessions(sessions)) => sessions,
            _ => panic!("Actor returned unexpected message"),
        }
    }
});

#[derive(Debug)]
pub struct Session {
    pub id: i32,
    pub realm_id: i32,
    pub name: String,
    pub utc_time: String,
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

    field utc_time() -> &str as "The time the session occurred" {
        &self.utc_time
    }

    field utc_created_at() -> &str as "The date the PlayerSession was created" {
        &self.utc_created_at
    }

    field player_sessions(&executor) -> Vec<PlayerSession> as "The list of Players who participated in this Session" {
        let result = query_db(executor, Messages::GetPlayerSessionsBySessionId(self.id));

        match result {
            Ok(Responses::PlayerSessions(player_sessions)) => player_sessions,
            _ => panic!("Actor returned unexpected message"),
        }
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

    field player(&executor) -> Player as "The Player who participated in the Session" {
        let result = query_db(executor, Messages::GetPlayerById(self.player_id));

        match result {
            Ok(Responses::Player(Some(player))) => player,
            Ok(Responses::Player(None)) => panic!("Player with ID from PlayerSession not found"),
            _ => panic!("Actor returned unexpected message"),
        }
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

    field walkout() -> i32 as "The amount of money the Player walked out with" {
        self.walkout
    }

    field utc_created_at() -> &str as "The date the PlayerSession was created" {
        &self.utc_created_at
    }
});
