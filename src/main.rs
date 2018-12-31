#[allow(proc_macro_derive_resolution_fallback)]
mod db;
mod handlers;

#[macro_use]
extern crate diesel;

use crate::db::DbExecutor;
use crate::handlers::{AppState, create_movie, delete_movie, get_movie};

use pretty_env_logger;
use actix;
use actix_web::{
    http, middleware, server, App
};
use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

fn main() {
    pretty_env_logger::init_custom_env("MOVIEDB_LOG");
    db::init_db("movies.db");

    let sys = actix::System::new("movie-db");

    let manager = ConnectionManager::<SqliteConnection>::new("movies.db");
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    // Start http server
    server::new(move || {
        vec! [
            App::with_state(AppState { db: addr.clone() })
                .prefix("/api")
                .middleware(middleware::Logger::default())
                .resource("/movie", |r| {
                    r.method(http::Method::POST).with(create_movie);
                    r.method(http::Method::DELETE).with(delete_movie);
                    r.method(http::Method::GET).with(get_movie)
                }),
        ]
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
