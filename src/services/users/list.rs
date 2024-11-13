use actix_web::{web::Data, HttpRequest, HttpResponse};

use crate::db::Db;

pub async fn user_list(req: HttpRequest, db: Data<Db>) -> HttpResponse {
    match db.get_ref().list_users().await {
        Ok(Some(users)) => HttpResponse::Ok().json(users),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("{}", e)),
    }
}