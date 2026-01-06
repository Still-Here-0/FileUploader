#![allow(unused)]

use axum::Router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod routes;

#[tokio::main]
async fn main() {
    
    init_log();
    dotenvy::dotenv().ok();
    
    log::info!("Starting app");

    let host = std::env::var("HOST").expect("'HOST' not valid on env");
    let port = std::env::var("PORT").expect("'PORT' not valid on env");

    let addr = format!("{host}:{port}");
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind address");
    
    let app = routes::app();
    axum::serve(listener, app).await.unwrap();
}

fn init_log() {
    unsafe {
        std::env::set_var("RUST_LOG", "debug"); 
        std::env::set_var("RUST_BACKTRACE", "1"); 
    };

    env_logger::init();
}

