use std::{env, fs};
use std::collections::HashMap;

use futures::StreamExt;
use tiberius::{AuthMethod, Client, Config, ExecuteResult, Query};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use super::{DBLoad};
use super::tiberius_interface::{WhereSqlValue, TiberiusCoversion, FromOwnedSql};

/* #region private funcitons */

fn build_select_clause<T, const U: usize>(top: Option<i32>, where_parameters: SqlParameter) -> String
    where T: DBLoad<U>
{
    let table_name = T::TAB;
    let where_clause = build_where_clause(where_parameters);
    let top = match top {
        Some(value) => format!("TOP {value}"),
        None => String::new(),
    };
    
    format!("SELECT {top} * FROM uploader.{table_name} {where_clause}")
}

fn build_where_clause(map: SqlParameter) -> String {
    let mut where_clause = String::new();

    for (key, value) in map {
        where_clause.push_str(" AND [");
        where_clause.push_str(key);
        where_clause.push_str("] ");

        match value {
            WhereSqlValue::Int(v) =>
                where_clause.push_str(&format!("= {v}")),

            WhereSqlValue::Float(v) =>
                where_clause.push_str(&format!("= {v}")),

            WhereSqlValue::Bool(v) =>
                where_clause.push_str(&format!("= {}", if v { 1 } else { 0 })),

            WhereSqlValue::Str(v, l) => {
                let value = match l {
                    true => &format!("like '{}'", escape(&v)),
                    false => &format!("= '{}'", escape(&v)),
                };
                where_clause.push_str(value)
            }

            WhereSqlValue::Bytes(v) =>
                where_clause.push_str(&format!("= 0x{}", hex::encode(v))),

            WhereSqlValue::Date(v) =>
                where_clause.push_str(&format!("= '{v}'")),

            WhereSqlValue::DateTime(v) =>
                where_clause.push_str(&format!("= '{v}'")),

            WhereSqlValue::IntList(list) =>
                where_clause.push_str(&format!("IN ({})",
                    list.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )),

            WhereSqlValue::StrList(list) =>
                where_clause.push_str(&format!("IN ({})",
                    list.iter()
                        .map(|v| format!("'{}'", escape(v)))
                        .collect::<Vec<_>>()
                        .join(", ")
                )),
        }
    }

    if (!where_clause.is_empty()) {
        let where_clause = where_clause[5..].to_string();
        return String::from(format!("WHERE {where_clause}"));
    }

    String::new()
}

fn escape(input: &str) -> String {
    input.replace("'", "''")
}

fn parse_query<'a>(sql: String) -> Query<'a> {

    let sql_result = fs::read_to_string(&sql);

    match sql_result {
        Ok(sql_file) => return Query::new(sql_file),
        Err(_) => return Query::new(sql),
    }
}

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
    
    let client = Client::connect(config, tcp.compat_write()).await?;
    
    Ok(client)
}

/* #endregion */

/* #region Types */

pub type SqlParameter = HashMap<&'static str, WhereSqlValue>;

/* #endregion */

pub async fn get_query_result(sql: String) -> anyhow::Result<ExecuteResult> {
    let query = parse_query(sql);
    let mut client = mssql_connect().await?;
    let result = query.execute(&mut client).await?;

    Ok(result)
}

pub async fn run_query(sql: String) -> anyhow::Result<usize> {
    let result = get_query_result(sql).await?;
    
    Ok(result.rows_affected().len())
}

pub async fn get_query_response<T, const U: usize>(sql: String, params: SqlParameter) -> anyhow::Result<Vec<T>> 
    where   
        T: DBLoad<U>,
{
    let query = parse_query(sql);
    let mut client = mssql_connect().await?;
    let mut stream = query.query(&mut client).await?;
    
    T::from_stream(stream).await
}

async fn get_query_single_response<R>(sql: String, column_name: Option<&'static str>) -> anyhow::Result<Option<R>>
    where R: TiberiusCoversion
{
    let query = parse_query(sql);
    let mut client = mssql_connect().await?;
    let mut stream= query.query(&mut client).await?;

    let row = stream.into_row().await?.expect("No data in the given query {query:?}");

    let value = match column_name {
        Some(name) => {
            let value= row.try_get_by_name(name)?;
            value
        },
        None => {
            let value = row.try_get_by_index(0)?;
            value
        },
    };
    
    Ok(value)
}

async fn select_from<T, const U: usize>(top: Option<i32>, where_parameters: SqlParameter) -> anyhow::Result<Vec<T>> 
    where T: DBLoad<U>
{
    let sql = build_select_clause::<T, U>(top, where_parameters);
    get_query_response(sql, SqlParameter::new()).await
}

async fn select_column_from<T, const U: usize, R>(top: Option<i32>, where_parameters: SqlParameter, column_name: &str) -> anyhow::Result<Vec<Option<R>>>
    where 
        T: DBLoad<U>,
        R: TiberiusCoversion
{
    let sql = build_select_clause::<T, U>(top, where_parameters);
    let query = parse_query(sql);

    let mut client = mssql_connect().await?;
    let mut stream= client.query("SELECT * FROM uploader.COLUMN_TYPE", &[]).await?;
    let mut row_stream = stream.into_row_stream();
    
    let mut result = Vec::new();
    while let Some(row) = row_stream.next().await.transpose()? {
        let value = row.try_get_by_name(column_name)?;
        result.push(value);
    }

    Ok(result)
}

/* #region TODO */

// TODO: need some work

// async fn select_single<T, const U: usize>(column_name: &str) -> anyhow::Result<SqlTypes>
//     where T: DBLoad<U>
// {
//     todo!()
// }

// async fn select_generic() -> anyhow::Result<HashMap<String, Vec<Vec<SqlTypes>>>> {
//     todo!()
// }

// async fn select_column_generic(column_name: &str) -> anyhow::Result<Vec<SqlTypes>> {
//     todo!()
// }

// async fn select_single_generic(column_name: &str) -> anyhow::Result<SqlTypes> {
//     todo!()
// }

async fn chain_executions() -> anyhow::Result<()> {
    todo!()
}

/* #endregion */

#[cfg(test)]
mod tests {
    use super::*;

    use futures::future::select;
    use crate::ddb::{DBLoad, tables::ColumnType};

    #[tokio::test]
    async fn a() {
        dotenvy::dotenv().ok();
        let mut where_p = SqlParameter::new();
        let sql = "SELECT * FROM uploader.COLUMN_TYPE".to_string();
        let cn = ColumnType::COL_SQL_TYPE;
        where_p.insert(ColumnType::COL_SQL_TYPE, WhereSqlValue::Str("INT".to_string(), true));
        const LEN: usize = ColumnType::COLS.len();
        let v = get_query_single_response::<String>(sql, Some(cn)).await.unwrap();
        // let v = select_column_from::<ColumnType, LEN, String>(
        //     None,
        //     where_p,
        //     ColumnType::COL_SQL_TYPE
        // ).await.unwrap();

        println!("{v:?}")
    }

    #[tokio::test]
    async fn test() {
        dotenvy::dotenv().ok();
        let mut client = mssql_connect().await.unwrap();
        let stream = client.query("SELECT name FROM sys.databases", &[]).await.unwrap();
        let mut rows = stream.into_row_stream();

        while let Some(row) = rows.next().await.transpose().unwrap() {
            // This works
            let name: Option<String> = row.try_get_by_index(0).unwrap();
            println!("{:?}", name);
        }
    }
}