//#![allow(unused)]

use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use rs_base_api::db::{self, ColumnType, DBLoad};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();
    log::info!("Starting app");

    let mut client = mssql_connect().await.unwrap();

    let rows = client.simple_query("SELECT * FROM DIGITAL_BRA_DEV.uploader.COLUMN_TYPE").await.unwrap();
    let result1 = rows.into_first_result().await.unwrap();
    log::info!("r1: {:?}", result1);

    let result2 = load_column_types().await.unwrap();
    log::info!("r2");
    for line in &result2 {
        log::info!("{:?}", line);
    }

    let json_response = serde_json::to_string(&result2)?;

    log::info!("json:\n{:?}", json_response);

    Ok(())
}

fn init_log() {
    unsafe {
        std::env::set_var("RUST_LOG", "debug"); 
        std::env::set_var("RUST_BACKTRACE", "1"); 
    };

    env_logger::init();
}

async fn mssql_connect() -> anyhow::Result<Client<tokio_util::compat::Compat<TcpStream>>> {
    let mut config = Config::new();
    config.host("****");
    config.port(0000);
    config.authentication(AuthMethod::sql_server("****", "****"));
    config.trust_cert(); // remove in production

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;

    Ok(client)
}

async fn load_column_types() -> anyhow::Result<Vec<db::ColumnType>> {
    let mut client = mssql_connect().await?;

    let stream = client
        .query("SELECT pk, SqlType, ViewType FROM DIGITAL_BRA_DEV.uploader.COLUMN_TYPE", &[],)
        .await?;

    ColumnType::from_row_stream(stream).await
}
