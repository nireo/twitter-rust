use crate::api_error::ApiError;
use crate::tweet::{Tweet, TweetMessage};
use crate::user::User;
use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/tweets")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let tweets = Tweet::find_all()?;
    Ok(HttpResponse::Ok().json(tweets))
}

#[get("/tweets/single/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let tweet = Tweet::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(tweet))
}

#[get("/tweets/{handle}")]
async fn find_handle(handle: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let tweets = Tweet::tweets_with_handle(handle.into_inner());
    Ok(HttpResponse::Ok().json(tweets))
}

#[post("/tweets")]
async fn create(
    tweet: web::Json<TweetMessage>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let id: Option<Uuid> = session.get("user_id")?;
    if let Some(_) = id {
        let tweet = Tweet::create(tweet.into_inner())?;
        Ok(HttpResponse::Ok().json(tweet))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

#[put("/tweets/{id}")]
async fn update(
    id: web::Path<Uuid>,
    tweet: web::Json<TweetMessage>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let session_id: Option<Uuid> = session.get("user_id")?;

    if let Some(_) = session_id {
        let user = User::find(session_id.unwrap())?;
        let existing_tweet = Tweet::find(id.into_inner())?;
        if user.handle != existing_tweet.handle {
            Err(ApiError::new(401, "Unauthorized".to_string()));
        }

        existing_tweet.update_self(tweet.into_inner())?;
        Ok(HttpResponse::Ok().json(existing_tweet))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/tweets/{id}}")]
async fn delete(id: web::Path<Uuid>, session: Session) -> Result<HttpResponse, ApiError> {
    let session_id: Option<Uuid> = session.get("user_id")?;

    if let Some(_) = session_id {
        // check for ownership
        let user = User::find(session_id.unwrap())?;
        let tweet = Tweet::find(id.into_inner())?;

        if user.handle != tweet.handle {
            Err(ApiError::new(401, "UnAuthorized".to_string()));
        }

        let num_deleted = tweet.delete_self();
        Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
