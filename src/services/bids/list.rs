use actix_web::{web::Data, HttpRequest, HttpResponse};

use crate::db::Db;

pub async fn bids_list(req: HttpRequest, db: Data<Db>) -> HttpResponse {
    match db.get_ref().list_bids().await {
        Ok(Some(bids)) => HttpResponse::Ok().json(bids),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("{}", e)),
    }
}
