#![allow(unused)]

use tokio_util::compat::TokioAsyncWriteCompatExt;
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use std::env;

use rs_base_api::db::{tables, db_traits::DBLoad};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    init_log();
    dotenvy::dotenv().ok();
    
    log::info!("Starting app");

    let mut client = mssql_connect().await.unwrap();

    let rows = client.simple_query("SELECT * FROM uploader.COLUMN_TYPE").await.unwrap();
    let result1 = rows.into_first_result().await.unwrap();
    log::info!("r1: {:?}", result1);

    // let result2 = load_column_types().await.unwrap();
    // log::info!("r2");
    // for line in &result2 {
    //     log::info!("{:?}", line);
    // }

    // let json_response = serde_json::to_string(&result2)?;

    // log::info!("json:\n{:?}", json_response);

    Ok(())
}

fn init_log() {
    unsafe {
        env::set_var("RUST_LOG", "debug"); 
        env::set_var("RUST_BACKTRACE", "1"); 
    };

    env_logger::init();
}

async fn mssql_connect() -> anyhow::Result<Client<tokio_util::compat::Compat<TcpStream>>> {
    let mut config = Config::new();

    let url = env::var("DB_URL")?;
    let port: u16 = env::var("DB_PORT")?.parse()?;
    let database = env::var("DB_NAME")?;
    let user = env::var("DB_USER")?;
    let password = env::var("DB_PASS")?;

    config.host(url);
    config.port(port);
    config.database(database);
    config.authentication(AuthMethod::sql_server(user, password));
    config.trust_cert(); // remove in production

    let tcp = TcpStream::connect(config.get_addr()).await?;
    // tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;

    Ok(client)
}

async fn load_column_types() -> anyhow::Result<Vec<tables::ColumnType>> {
    let mut client = mssql_connect().await?;

    let stream = client
        .query("SELECT * FROM uploader.COLUMN_TYPE", &[],)
        .await?;

    tables::ColumnType::from_row_stream(stream).await
}
