use craete::api_error::ApiError;
use crate::tweet::{Tweet, TweetMessage};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/tweets")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let tweets = Tweet::find_all()?;
    Ok(HttpResponse::Ok().json(tweets))
}

#[get("/tweets/single/{id}")]
async fn find(id: web::Path::<Uuid>) -> Result<HttpResponse, ApiError> {
    let tweet = Tweet::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(tweet))
}

#[post("/tweets")]
async fn create(tweet: web::Json<TweetMessage>) -> Result<HttpResponse, ApiError> {
    let tweet = Tweet::create(tweet.into_inner())?;
    Ok(HttpResponse::Ok().json(tweet))
}

#[get("/tweets/user/{username}}")]
async fn tweets_by_user(handle: web::Path::<String>) -> Result<HttpResponse, ApiError> {
    let tweets = Tweet::find_tweets_by_user(handle);
    Ok(HttpResponse::Ok().json(tweets))
}

#[put("/tweets/{id}")]
async fn update(id: web::Path<Uuid>, user: web::Json<TweetMessage>) -> Result<HttpResponse, ApiError> {
    let tweet = Tweet::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(tweet))
}

#[delete("/tweets/{id}}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Tweet::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}

