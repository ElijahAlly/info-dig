use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::statements;
use crate::models::enums::RatingType;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Statement {
    pub statement_id: i32,
    pub user_id: i32,
    pub slug: String,
    pub content: String,
    pub context: Option<String>,
    pub public_rating: Option<String>,
    pub our_rating: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "statements"]
pub struct NewStatement {
    pub user_id: i32,
    pub slug: String,
    pub content: String,
    pub context: Option<String>,
    pub public_rating: Option<String>,
    pub our_rating: Option<String>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone)]
#[table_name = "statements"]
pub struct UpdateStatement {
    pub content: Option<String>,
    pub slug: Option<String>, // FIX: The slug will also need to be updated
    pub context: Option<String>,
    pub public_rating: Option<RatingType>, // FIX: not working 
    pub our_rating: Option<RatingType>, // FIX: not working
}