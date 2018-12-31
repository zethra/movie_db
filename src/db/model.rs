use super::schema::*;

use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable)]
#[table_name = "movies"]
#[primary_key(movies_id)]
pub struct Movie {
    #[column_name = "movies_id"]
    pub id: String,
    #[column_name = "movies_name"]
    pub name: String,
    #[column_name = "movies_rating"]
    pub rating: String,
    #[column_name = "movies_category"]
    pub category: String,
    #[column_name = "movies_format"]
    pub format: String,
    #[column_name = "movies_aspect"]
    pub aspect: String,
    #[column_name = "movies_actors"]
    pub actors: String,
    #[column_name = "movies_studio_id"]
    pub studio_id: String,
}
