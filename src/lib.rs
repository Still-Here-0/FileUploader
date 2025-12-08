#![allow(unused)]

mod ddb;
pub mod api;

mod macros;

use actix_web::{ HttpResponse, Responder, get };

#[get("/")]
pub async fn api_scream() -> impl Responder {
    HttpResponse::Forbidden()
}
