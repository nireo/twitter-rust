use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{NaiveDateTime};
use crate::db;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "tweet"]
pub struct TweetMessage {
    pub content: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "tweet"]
pub struct Tweet {
    pub id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}


