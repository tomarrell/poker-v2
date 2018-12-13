use actix::prelude::*;
use actix_web::Error;
use futures::Future;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

pub mod entities;
pub mod schema;
pub mod input_types;

use self::schema::Schema;
use crate::db::{DBExecutor, Messages, Responses};

// Context to be shared amongst GraphQL Requests
pub struct Context {
    db: Addr<DBExecutor>,
}

impl juniper::Context for Context {}

// Setup GraphQL Executor Actor
pub struct GraphQLExecutor {
    schema: Arc<Schema>,
    db: Addr<DBExecutor>,
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl GraphQLExecutor {
    pub fn new(schema: Arc<Schema>, db_exe: Addr<DBExecutor>) -> GraphQLExecutor {
        GraphQLExecutor {
            schema: schema,
            db: db_exe,
        }
    }
}

// Setup GraphQL data message to executor actor
#[derive(Serialize, Deserialize)]
pub struct GraphQLData(juniper::http::GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    // Executor will always return serialized JSON response
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _ctx: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(
            &self.schema,
            &Context {
                db: self.db.clone(),
            },
        );
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

pub fn query_db(
    executor: &&juniper::Executor<'_, Context>,
    message: Messages,
) -> Result<Responses, String> {
    executor
        .context()
        .db
        .send(message)
        .wait()
        .map_err(|res| {
            format!("Message failed delivery due to mailbox closed or timeout: {}", res)
        })?
        .map_err(|res| {
            format!("Failed to query DB: {}", res)
        })
}
