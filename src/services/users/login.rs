use actix_web::{
    web::{self, Data},
    HttpResponse,
};

use crate::{db::Db, dto};

pub async fn login(req: web::Json<dto::login_dto::LoginDto>, db: Data<Db>) -> HttpResponse {
    match db
        .get_ref()
        .find_by_login_and_password(req.0.login(), req.0.password())
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("Пользователь не найден"),
        Err(e) => HttpResponse::InternalServerError().json(format!("{}", e)),
    }
}
