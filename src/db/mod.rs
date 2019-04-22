pub mod model;
pub mod schema;

use ::actix::prelude::*;
use actix_web::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use log::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

include!(concat!(env!("OUT_DIR"), "/db_setup.rs"));

pub fn init_db(db_url: &str) {
    debug!("DB URL: {}", db_url);
    let conn = SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
    for file_name in DB_SETUP.file_names() {
        let mut file = DB_SETUP
            .read(file_name)
            .unwrap_or_else(|_| panic!("Unable to load db init file: {}", file_name));
        let mut query = String::new();
        file.read_to_string(&mut query)
            .unwrap_or_else(|_| panic!("Unable to load db init query: {}", file_name));
        debug!("Initializing table: \n{}", query);
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
            .map_err(|_| error::ErrorInternalServerError("Error inserting movie"))?;

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
            .map_err(|_| error::ErrorInternalServerError("Error deleting movie"))?;

        Ok(())
    }
}

/*
 * Get movie
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMovie {
    pub id: String,
}

impl Message for GetMovie {
    type Result = Result<model::Movie, Error>;
}

impl Handler<GetMovie> for DbExecutor {
    type Result = Result<model::Movie, Error>;

    fn handle(&mut self, msg: GetMovie, _: &mut Self::Context) -> Self::Result {
        use self::schema::movies::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let mut items = movies
            .filter(movies_id.eq(msg.id))
            .limit(1)
            .load::<model::Movie>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error getting movie"))?;

        items
            .pop()
            .ok_or(error::ErrorInternalServerError("No movie with that id"))
    }
}

/*
 * Update new movie
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMovie {
    pub id: String,
    pub title: String,
    pub rating: String,
    pub category: String,
    pub format: String,
    pub aspect: String,
    pub actors: String,
    pub drawer: String,
    pub column: String,
}

impl Message for UpdateMovie {
    type Result = Result<(), Error>;
}

impl Handler<UpdateMovie> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: UpdateMovie, _: &mut Self::Context) -> Self::Result {
        use self::schema::movies::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let target = movies.filter(movies_id.eq(msg.id));
        diesel::update(target)
            .set((
                movies_title.eq(msg.title),
                movies_rating.eq(msg.rating),
                movies_category.eq(msg.category),
                movies_format.eq(msg.format),
                movies_aspect.eq(msg.aspect),
                movies_actors.eq(msg.actors),
                movies_drawer.eq(msg.drawer),
                movies_column.eq(msg.column),
            ))
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting movie"))?;

        Ok(())
    }
}

/*
 * Get all movies
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllMovies;

impl Message for GetAllMovies {
    type Result = Result<Vec<model::Movie>, Error>;
}

impl Handler<GetAllMovies> for DbExecutor {
    type Result = Result<Vec<model::Movie>, Error>;

    fn handle(&mut self, _: GetAllMovies, _: &mut Self::Context) -> Self::Result {
        use self::schema::movies::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        let items = movies
            .load::<model::Movie>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error getting all movies"))?;

        Ok(items)
    }
}
