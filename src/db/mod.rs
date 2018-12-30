pub mod schema;
pub mod model;

use log::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use ::actix::prelude::*;
use actix_web::*;
use uuid::Uuid;

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
pub struct CreateMovie {
    pub name: String,
}

impl Message for CreateMovie {
    type Result = Result<model::Movie, Error>;
}

impl Handler<CreateMovie> for DbExecutor {
    type Result = Result<model::Movie, Error>;

    fn handle(&mut self, msg: CreateMovie, _: &mut Self::Context) -> Self::Result {
        use self::schema::movies::dsl::*;

        let uuid = Uuid::new_v4().to_string();
        let new_movie = model::Movie {
            id: uuid,
            name: msg.name.clone(),
            rating: String::new(),
            category: String::new(),
            format: String::new(),
            aspect: String::new(),
            actors: String::new(),
            studio_id: String::new(),
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(movies)
            .values(&new_movie)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;

        Ok(new_movie)
    }
}

/*
 * Create a new studio
 */
pub struct CreateStudio {
    pub name: String,
}

impl Message for CreateStudio {
    type Result = Result<model::Studio, Error>;
}

impl Handler<CreateStudio> for DbExecutor {
    type Result = Result<model::Studio, Error>;

    fn handle(&mut self, msg: CreateStudio, _: &mut Self::Context) -> Self::Result {
        use self::schema::studios::dsl::*;

        let uuid = Uuid::new_v4().to_string();
        let new_studio = model::Studio {
            id: uuid,
            name: msg.name.clone(),
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(studios)
            .values(&new_studio)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting studio"))?;

        Ok(new_studio)
    }
}
