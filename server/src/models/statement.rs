use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::statements;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Statement {
    pub statement_id: i32,
    pub user_id: i32,
    pub slug: String,
    pub content: String,
    pub context: Option<String>,
    pub public_rating: Option<String>,
    pub our_rating: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
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
    pub slug: Option<String>, // FIX: The slug will also need to be updated
    pub content: Option<String>,
    pub context: Option<String>,
    pub public_rating: Option<String>, // FIX: not working 
    pub our_rating: Option<String>, // FIX: not working
}
