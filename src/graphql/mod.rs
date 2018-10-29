use actix::prelude::*;
use actix_web::Error;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

pub mod entities;
mod resolvers;
pub mod schema;

use super::db::DBExecutor;
use self::schema::Schema;

// Setup GraphQL Executor Actor
pub struct GraphQLExecutor {
    schema: Arc<Schema>,
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl GraphQLExecutor {
    pub fn new(schema: Arc<Schema>, db_exe: Addr<DBExecutor>) -> GraphQLExecutor {
        GraphQLExecutor { schema: schema }
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

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &());
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}
