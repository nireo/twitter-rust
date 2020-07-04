use crate::api_error::ApiError;
use crate::user::{User, UserMessage};
use actix_web::{post, get, web, HttpResponse};

#[post("/register")]
async fn register(user: web::Json<Username>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
}
