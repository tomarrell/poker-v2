#![warn(rust_2018_idioms)]

#[macro_use]
extern crate juniper;

use actix::{Addr, SyncArbiter};
use actix_web::{
    http, server, App, AsyncResponder, Error, FutureResponse, HttpRequest, HttpResponse, Json,
    State,
};
use futures::future::Future;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

mod db;
mod graphql;

use self::db::DBExecutor;
use self::graphql::schema::create_schema;
use self::graphql::{GraphQLData, GraphQLExecutor};

const ADDRESS: &str = "localhost:8088";
const DB_PATH: &str = "./poker.db";
const NUM_THREADS: usize = 3;

struct AppState {
    graphql_exe: Addr<GraphQLExecutor>,
}

fn graphiql(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let html = juniper::http::graphiql::graphiql_source(&(format!("http://{}/graphql", ADDRESS)));

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

fn graphql((state, data): (State<AppState>, Json<GraphQLData>)) -> FutureResponse<HttpResponse> {
    state
        .graphql_exe
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(resp) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(resp)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

fn main() {
    // SQLite connection pool initialization, with
    // PRAGMA set on connection creation
    let manager = SqliteConnectionManager::file(DB_PATH)
        .with_init(|c| c.execute_batch("PRAGMA foreign_keys=1;"));
    let pool = r2d2::Pool::new(manager).unwrap();

    // Actor root system
    let system = actix::System::new("poker-v2");

    // Start DB Actor
    let db_addr = SyncArbiter::start(NUM_THREADS, move || DBExecutor(pool.clone()));
    let graphql_db_addr = db_addr.clone();

    // Graphql Actor setup
    let schema = Arc::new(create_schema());
    let graphql_addr = SyncArbiter::start(NUM_THREADS, move || {
        GraphQLExecutor::new(schema.clone(), graphql_db_addr.clone())
    });

    // Actix-web Server routing
    server::new(move || {
        App::with_state(AppState {
            graphql_exe: graphql_addr.clone(),
        })
        .resource("/graphql", |r| r.method(http::Method::POST).with(graphql))
        .resource("/graphiql", |r| r.method(http::Method::GET).h(graphiql))
    })
    .bind(ADDRESS)
    .unwrap()
    .start();

    println!("Started http server on: {}", ADDRESS);
    println!("GraphiQL panel serving at http://{}/graphiql", ADDRESS);
    let _ = system.run();
}
