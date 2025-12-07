#![allow(unused)]

use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    init_log();
    dotenvy::dotenv().ok();
    
    log::info!("Starting app");

    // let host = std::env::var("HOST").unwrap();
    // let port: u16  = std::env::var("PORT").unwrap().parse().unwrap();
    // log::info!("App will run on http://{host}:{port}");
    
    // HttpServer::new( move || {
    //     let logger = Logger::default();
    //     App::new()
    //         .wrap(logger)
    //         .service(file_uploader::api_scream)
    // })
    // .bind( (host, port) )?
    // .run().await;

    Ok(())
}

fn init_log() {
    unsafe {
        std::env::set_var("RUST_LOG", "debug"); 
        std::env::set_var("RUST_BACKTRACE", "1"); 
    };

    env_logger::init();
}

