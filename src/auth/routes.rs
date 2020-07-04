use crate::api_error::ApiError;
use crate::user::{User, UserMessage};
use actix_web::{post, get, web, HttpResponse};
use uuid::Uuid;
use serde_json::json;
use actix_session::Session;

#[post("login")]
async fn login(credentials: web::Json<UserMessage>, session: Session) -> Result<HttpResponse, ApiError> {
    let credentials = credentials.into_inner();

    let user = User::find_by_email(credentials.email)
        .map_err(|e| {
            match e.status_code {
                404 => ApiError::new(401, "Credentials not valid".to_string()),
                _ => e,
            }
        })?;

    let is_valid = user.verify_password(credentials.password.as_bytes())?;

    if is_valid == true {
        session.set("user_id", user.id)?;
        session.renew();

        Ok(HttpResponse::Ok().json(user))
    }
    else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

#[post("logout")]
async fn logout(session: Session) -> Result<HttpResponse, ApiError> {
    let id: Option<Uuid> = session.get("user_id")?;
    if let Some(_) = id {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({"message": "Successfully signed out"})))
    }
    else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/register")]
async fn register(user: web::Json<Username>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(logout);
}
