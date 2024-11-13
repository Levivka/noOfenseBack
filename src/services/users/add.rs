use actix_web::{web::{self, Data}, HttpResponse};
use uuid::Uuid;

use crate::{db::Db, dto, models};

pub async fn user_add(req: web::Json<dto::user_dto::UserDto>, db: Data<Db>) -> HttpResponse {
    match db.get_ref().add_user(&models::user::User::new(
        Uuid::new_v4().to_string(),
        req.0.fio().to_string(),
        req.0.login().to_string(),
        req.0.password().to_string(),
    )).await {
        Ok(res) => HttpResponse::Created().json(res),
        Err(e) => HttpResponse::InternalServerError().json(format!("{}", e)),
    }
}