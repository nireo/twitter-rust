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

impl Tweet {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;
        let tweets = tweet::table.load::<Tweet>(&conn)?;

        Ok(tweets)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let tweet = tweet::table
            .filter(tweet::id.eq(id))
            .first(&conn)?;

        Ok(tweet)
    }

    pub fn create(tweet: TweetMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let tweet = Tweet::from(tweet);
        let tweet = diesel::insert_into(user::table)
            .values(tweet)
            .get_results(&conn)?;

        Ok(tweet)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;
        let res = diesel::delete(tweet::table.filter(tweet::id.eq(id))).execute(&conn)?;

        Ok(res)
    }

    pub fn update(id: Uuid, tweet: TweetMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let tweet = diesel::update(tweet::table)
            .filter(tweet::id.eq(id))
            .set(tweet)
            .get_result(&conn)?;

        Ok(tweet)
    }
}

impl From<UserMessage> for Tweet {
    fn from(tweet: TweetMessage) -> Self {
        Tweet {
            id: Uuid::new_v4(),
            content: tweet.content,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
