use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::proposals;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Proposal {
    pub proposal_id: i32,
    pub user_id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub status: Option<String>,
    pub yeas: i32,
    pub nays: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "proposals"]
pub struct NewProposal {
    pub user_id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub status: Option<String>,
    pub yeas: i32,
    pub nays: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset, Deserialize, Debug, Clone)]
#[table_name = "proposals"]
pub struct UpdateProposal {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub yeas: Option<i32>,
    pub nays: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
