pub mod schema;
pub mod model;

use log::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use ::actix::prelude::*;
use actix_web::*;
use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};

include!(concat!(env!("OUT_DIR"), "/db_setup.rs"));

pub fn init_db(db_url: &str) {
    debug!("DB URL: {}", db_url);
    let conn = SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
    for file_name in DB_SETUP.file_names() {
        let mut file = DB_SETUP.read(file_name)
            .unwrap_or_else(|_| panic!("Unable to load db init file: {}", file_name));
        let mut query = String::new();
        file.read_to_string(&mut query)
            .unwrap_or_else(|_| panic!("Unable to load db init query: {}", file_name));
        debug!("Initalizing table: \n{}", query);
        ::diesel::sql_query(query)
            .execute(&conn)
            .expect("Fail to init db");
    }
    info!("Database initialized");
}

pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

/*
 * Create a new movie
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMovie {
    pub title: String,
    pub rating: String,
    pub category: String,
    pub format: String,
    pub aspect: String,
    pub actors: String,
    pub drawer: String,
    pub column: String,
}

impl Message for CreateMovie {
    type Result = Result<(), Error>;
}

impl Handler<CreateMovie> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: CreateMovie, _: &mut Self::Context) -> Self::Result {
        use self::schema::movies::dsl::*;

        let uuid = Uuid::new_v4().to_string();
        let new_movie = model::Movie {
            id: uuid,
            title: msg.title.clone(),
            rating: msg.rating.clone(),
            category: msg.category.clone(),
            format: msg.format.clone(),
            aspect: msg.aspect.clone(),
            actors: msg.actors.clone(),
            drawer: msg.drawer.clone(),
            column: msg.column.clone(),
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(movies)
            .values(&new_movie)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;

        Ok(())
    }
}

/*
 * Delete movie
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMovie {
    pub id: String,
}

impl Message for DeleteMovie {
    type Result = Result<(), Error>;
}

impl Handler<DeleteMovie> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteMovie, _: &mut Self::Context) -> Self::Result {
        use self::schema::movies::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::delete(movies.filter(movies_id.eq(msg.id)))
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;

        Ok(())
    }
}
