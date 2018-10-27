#[macro_use]
extern crate juniper;
extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate serde_derive;
extern crate serde_json;

use std::thread;
use r2d2_sqlite::SqliteConnectionManager;
use actix::prelude::*;
use actix_web::{
    http, server, App, AsyncResponder, Error, FutureResponse, HttpRequest, HttpResponse, Json,
    State,
};
use futures::future::Future;
use rusqlite::Connection;
use serde_derive::{Deserialize, Serialize};

mod db;
mod resolvers;
mod schema;

use schema::{create_schema, Schema};

const ADDRESS: &'static str = "localhost:8088";
const DB_PATH: &'static str = "./poker-v2.db";

struct AppState {
    executor: Addr<GraphQLExecutor>,
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

struct GraphQLExecutor {
    schema: std::sync::Arc<Schema>,
}

impl GraphQLExecutor {
    fn new(schema: std::sync::Arc<Schema>) -> GraphQLExecutor {
        GraphQLExecutor { schema: schema }
    }
}

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

fn graphiql(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let html = juniper::http::graphiql::graphiql_source(&(format!("http://{}/graphql", ADDRESS)));

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &());
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(juniper::http::GraphQLRequest);

fn graphql((st, data): (State<AppState>, Json<GraphQLData>)) -> FutureResponse<HttpResponse> {
    st.executor
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }).responder()
}

fn respond_ok(_req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn main() {
    // SQLite connection
    let manager = SqliteConnectionManager::file(DB_PATH);
    let pool = r2d2::Pool::new(manager).unwrap();

    // Actor root system
    let system = actix::System::new("poker-v2");
    let schema = std::sync::Arc::new(create_schema());
    let addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

    server::new(move || {
        App::with_state(AppState {
            executor: addr.clone(),
        }).resource("/graphql", |r| r.method(http::Method::POST).with(graphql))
        .resource("/graphiql", |r| r.method(http::Method::GET).h(graphiql))
    }).bind(ADDRESS)
    .unwrap()
    .start();

    println!("Started http server on: {}", ADDRESS);
    let _ = system.run();
}
