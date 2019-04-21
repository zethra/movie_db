use moviedb::{
    self,
    db,
    db::DbExecutor,
    handlers::{
        create_movie, delete_movie, get_all_movies, get_movie, update_movie, AppState,
    },
};

use actix;
use actix::prelude::*;
use actix_web::{fs, http, middleware, server, App};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use pretty_env_logger;

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
        vec![
            App::with_state(AppState { db: addr.clone() })
                .prefix("/api")
                .middleware(middleware::Logger::default())
                .resource("/movie", |r| {
                    r.method(http::Method::POST).with(create_movie);
                    r.method(http::Method::DELETE).with(delete_movie);
                    r.method(http::Method::GET).with(get_movie);
                    r.method(http::Method::PUT).with(update_movie);
                })
                .resource("/all_movies", |r| {
                    r.method(http::Method::GET).with(get_all_movies)
                }),
            App::with_state(AppState { db: addr.clone() }).handler(
                "/",
                fs::StaticFiles::new("./static")
                    .unwrap()
                    .index_file("./index.html"),
            ),
        ]
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: http://127.0.0.1:8080");
    let _ = sys.run();
}
