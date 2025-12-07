#![allow(unused)]

use tokio_util::compat::TokioAsyncWriteCompatExt;
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use std::env;

mod ddb;
pub mod api;

mod macros;

use actix_web::{ HttpResponse, Responder, get };

#[get("/")]
pub async fn api_scream() -> impl Responder {
    HttpResponse::Forbidden()
}
