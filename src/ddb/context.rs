use std::env;

use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::ddb::DBLoad;

pub async fn mssql_connect() -> anyhow::Result<Client<Compat<TcpStream>>> {
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
