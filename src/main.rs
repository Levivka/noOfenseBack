use actix_web::{
    body::None,
    http::Method,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use db::Db;
use dto::user_dto::UserDto;
use futures::stream::StreamExt;
use models::user::{self, User};
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson, Bson, Document, Uuid},
    error::Error,
    Client, Collection,
};
use std::net::Ipv4Addr;

mod db;
mod dto;
mod models;
mod services;
mod sockets;

const MONGOURL: &str = "mongodb://levik:123@mongo.desq.com.ru/dnd-helper";
const PORT: &str = "8083";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let http_url: String = format!("{}:{}", Ipv4Addr::LOCALHOST, 8081);
    // let ip: Vec<&str> = http_url.split(":").collect();
    println!("HTTP SERVER on ip: {} port: {}", Ipv4Addr::LOCALHOST, PORT);
    let db: Data<Db> = Data::new(Db::init(MONGOURL).await.expect("Error while connect to db"));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&db))
            .service(
                web::resource("/users/list").route(web::get().to(services::users::list::user_list)),
            )
            .service(
                web::resource("/users/add").route(web::post().to(services::users::add::user_add)),
            )
            .service(web::resource("/login").route(web::post().to(services::users::login::login)))
    })
    .bind(format!("{}:{}", Ipv4Addr::LOCALHOST, PORT))?
    .run()
    .await
}
