use std::{env, fs, result};

use chrono::{NaiveDate, NaiveDateTime};
use futures::StreamExt;
use serde::de::value;
use tiberius::{AuthMethod, Client, Config, ExecuteResult, Query};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use super::db_types::{SqlParameters, SqlValue, ChainedExec};
use super::super::{DBLoad};
use super::tiberius_interface::{TiberiusCoversion, FromOwnedSql};

/* #region PRIVATE FUNCTIONS */

fn build_where_clause(map: SqlParameters) -> String {
    let mut where_clause = String::new();

    for (key, value) in map {
        where_clause.push_str("AND [");
        where_clause.push_str(&key);
        where_clause.push_str("] ");

        where_clause.push_str(&value.to_sql_where());
    }

    String::from(format!("WHERE 1 = 1 {where_clause}"))
}

fn build_known_select_clause<T>(where_parameters: SqlParameters, columns: Option<Vec<&str>>, top: Option<u8>) -> String
    where 
        T: DBLoad
{
    let table_name = T::TAB;
    
    build_generic_select_clause(table_name, where_parameters, columns, top)
}

fn build_generic_select_clause(table_name: &str, where_parameters: SqlParameters, columns: Option<Vec<&str>>, top: Option<u8>) -> String {
    
    let where_clause = build_where_clause(where_parameters);
    let top = match top {
        Some(value) => format!("TOP {value}"),
        None => String::new(),
    };

    let columns = match columns {
        Some(values) => values.join(", "),
        None => "*".to_string(),
    };
    
    format!("SELECT {top} {columns} FROM uploader.[{table_name}] {where_clause}")
}

fn parse_query<'a>(sql: String) -> Query<'a> {
    let path = env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let queries_path = format!("{path}/queries");

    let sql_result = fs::read_to_string(format!("{queries_path}/{sql}.sql"));

    match sql_result {
        Ok(sql_file) => return Query::new(sql_file),
        Err(_) => return Query::new(sql),
    }
}

/* #endregion */

/* #region PUBLIC FUNCITONS */

pub async fn mssql_client() -> anyhow::Result<Client<Compat<TcpStream>>> {
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

pub async fn get_query_result(sql: String) -> anyhow::Result<ExecuteResult> {
    let query = parse_query(sql);
    let mut client = mssql_client().await?;
    let result = query.execute(&mut client).await?;

    Ok(result)
}

pub async fn run_query(sql: String) -> anyhow::Result<Vec<u64>> {
    let result = get_query_result(sql).await?;
    
    Ok(result.rows_affected().to_vec())
}

pub async fn get_response_from<T>(sql: String, params: SqlParameters) -> anyhow::Result<Vec<T>> 
    where 
        T: DBLoad,
{
    let query = parse_query(sql);
    let mut client = mssql_client().await?;
    let mut stream = query.query(&mut client).await?;
    
    T::from_stream(stream).await
}

pub async fn get_single_response_from<R>(sql: String, column_name: Option<&str>) -> anyhow::Result<Option<R>>
    where 
        R: TiberiusCoversion
{
    let query = parse_query(sql);
    let mut client = mssql_client().await?;
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

pub async fn select_from<T>(where_parameters: SqlParameters, columns: Option<Vec<&str>>, top: Option<u8>) -> anyhow::Result<Vec<T>> 
    where 
        T: DBLoad
{
    let sql = build_known_select_clause::<T>(where_parameters, columns, top);
    get_response_from(sql, SqlParameters::new()).await
}

pub async fn select_column_from<T, R>(column_name: &str, where_parameters: SqlParameters, top: Option<u8>) -> anyhow::Result<Vec<Option<R>>>
    where 
        T: DBLoad,
        R: TiberiusCoversion
{
    let columns = Some(vec![column_name]);
    let sql = build_known_select_clause::<T>(where_parameters, columns, top);
    let query = parse_query(sql);

    let mut client = mssql_client().await?;
    let mut stream= query.query(&mut client).await?;
    let mut row_stream = stream.into_row_stream();
    
    let mut result = Vec::new();
    while let Some(row) = row_stream.next().await.transpose()? {
        let value = row.try_get_by_name(column_name)?;
        result.push(value);
    }

    Ok(result)
}

pub async fn select_single_from<T, R>(where_parameters: SqlParameters, column_name: &str) -> anyhow::Result<Option<R>>
    where 
        T: DBLoad,
        R: TiberiusCoversion
{
    let sql = build_known_select_clause::<T>(where_parameters, None, None);
    let query = parse_query(sql);

    let mut client = mssql_client().await?;
    let mut stream= query.query(&mut client).await?;
    let mut row_stream = stream.into_row_stream();
    
    let mut result = match row_stream.next().await.transpose()? {
        Some(value) => value.try_get_by_name(column_name)?,
        None => None,
    };

    Ok(result)
}

pub async fn get_generic_response(sql: String) -> anyhow::Result<Vec<tiberius::Row>> {
    let query = parse_query(sql);
    let mut client = mssql_client().await?;
    let mut stream = query.query(&mut client).await?;
    let mut row_stream = stream.into_row_stream();

    let mut result = Vec::new();

    while let Some(row) = row_stream.next().await.transpose()? as Option<tiberius::Row> {
        result.push(row);

    }

    Ok(result)
}

pub async fn select_generic(table_name: &str, where_parameters: SqlParameters, columns: Option<Vec<&str>>, top: Option<u8>) -> anyhow::Result<Vec<tiberius::Row>> {
    let sql = build_generic_select_clause(table_name, where_parameters, columns, top);

    get_generic_response(sql).await
}

async fn chain_executions(chain_exec: ChainedExec<'_>, parameters: &mut SqlParameters) -> anyhow::Result<Vec<u64>> {
    let mut rows_affected = Vec::<u64>::new();

    let mut  client = mssql_client().await?;
    client.simple_query("BEGIN TRANSACTION").await?;

    let result = (|| async {
        for exec in chain_exec {
            let (sql, new_parameter_name) = exec(&parameters);
            let query = parse_query(sql);
    
            match new_parameter_name {
                Some(name) => {
                    let steam = query.query(&mut client).await?;
                    let mut row_strem = steam.into_row_stream();
                    let mut partial_rows_affected = 0;
                    
                    while let Some(a) = row_strem.next().await.transpose()? {
                        partial_rows_affected += 1;
                        // TODO: in the future values beside INT should be accepted in this function
                        let opt = a.try_get_by_index::<i32>(0)?;
    
                        if let Some(value) = opt {
                            parameters.insert(name.clone(), SqlValue::Int(value));
                        }
                    }
                },
                None => {
                    let partial_result = query.execute(&mut client).await?;
                    rows_affected.append(&mut partial_result.rows_affected().to_vec());
                },
            }
        }
        Ok::<Vec<u64>, anyhow::Error>(rows_affected)
    });

    match result().await {
        Ok(affected) => {
            client.simple_query("COMMIT").await?;
            Ok(affected)
        }
        Err(e) => {
            client.simple_query("ROLLBACK").await?;
            Err(e)
        }
    }
}

/* #endregion */


#[cfg(test)]
mod tests {
    use actix_web::web::delete;

    use super::*;
    use crate::ddb::{DBLoad, context::db_types::{ChainReturn, SqlValue}, tables::*};
    const VAR_TYPES: usize = 6;

    // #[tokio::test]
    async fn not_a_test() {
        dotenvy::dotenv().ok();
        let mut chain_exec = ChainedExec::new();
        chain_exec.push(&init_insert);
        chain_exec.push(&next_insert);
        chain_exec.push(&next_insert);
        let mut p = SqlParameters::new();

        let mut  client = mssql_client().await.unwrap();
        // client.simple_query("BEGIN TRANSACTION").await.unwrap();
        // client.query("INSERT INTO uploader.COLUMN_TYPE (SqlType, ViewType) VALUES ('asdaf', 'sadfas')", &[]).await.unwrap();
        // client.simple_query("ROLLBACK").await.unwrap();

        let v = chain_executions(chain_exec, &mut p).await;

        println!("{v:?}")
    }

    fn init_insert(parameters: &SqlParameters) -> ChainReturn {
        (
            "INSERT INTO uploader.COLUMN_TYPE (SqlType, ViewType) OUTPUT INSERTED.pk VALUES ('asdaf', 'sadfas')".to_string(),
            Some("pk".to_string())
        )
    }
    
    fn next_insert(parameters: &SqlParameters) -> ChainReturn {
        let pk = parameters.get("pk").unwrap().to_string();
        (
            format!("INSERT INTO uploader.COLUMN_TYPE (SqlType, ViewType) OUTPUT INSERTED.pk VALUES ('{pk}', '{pk}')"),
            Some("pk".to_string())
        )
    }

    #[test]
    fn check_build_where() {
        let mut a = SqlParameters::new();
        a.insert("Int".to_string(), SqlValue::Int(2));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [Int] = 2".to_string());

        let mut a = SqlParameters::new();
        a.insert("Float".to_string(), SqlValue::Float(2.5));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [Float] = 2.5".to_string());

        let mut a = SqlParameters::new();
        a.insert("Bool".to_string(), SqlValue::Bool(true));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [Bool] = 1".to_string());

        let mut a = SqlParameters::new();
        a.insert("StringL".to_string(), SqlValue::Str(String::from("OLOKO"), true));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [StringL] like 'OLOKO'".to_string());
        
        let mut a = SqlParameters::new();
        a.insert("StringN".to_string(), SqlValue::Str(String::from(";-;"), false));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [StringN] = '-'".to_string());

        let mut a = SqlParameters::new();
        a.insert("Date".to_string(), SqlValue::Date(NaiveDate::parse_from_str("2025-12-21", "%Y-%m-%d").unwrap()));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [Date] = '2025-12-21'".to_string());

        let mut a = SqlParameters::new();
        a.insert("DateTime".to_string(), SqlValue::DateTime(NaiveDateTime::parse_from_str("2025-12-21 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap()));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [DateTime] = '2025-12-21 10:30:00'".to_string());
        
        let mut a = SqlParameters::new();
        a.insert("IntVec".to_string(), SqlValue::IntList(vec![1, 2, 3]));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [IntVec] IN (1, 2, 3)".to_string());

        let mut a = SqlParameters::new();
        a.insert("FloatVec".to_string(), SqlValue::FloatList(vec![1.5, 2.5, 3.5]));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [FloatVec] IN (1.5, 2.5, 3.5)".to_string());

        let mut a = SqlParameters::new();
        a.insert("StrVec".to_string(), SqlValue::StrList(vec!["1".to_string(), "2".to_string(), "3".to_string()]));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, "WHERE 1 = 1 AND [StrVec] IN ('1', '2', '3')".to_string());
    }

    #[test]
    fn check_build_select() {
        let build = build_known_select_clause::<ColumnType>(SqlParameters::new(), None, None);
        assert_eq!(build, "SELECT  * FROM uploader.[COLUMN_TYPE] WHERE 1 = 1 ");
        
        let build = build_known_select_clause::<ColumnType>(SqlParameters::new(), None, Some(10));
        assert_eq!(build, "SELECT TOP 10 * FROM uploader.[COLUMN_TYPE] WHERE 1 = 1 ");
        
        let mut a = SqlParameters::new();
        a.insert("Int".to_string(), SqlValue::Int(2));
        let build = build_known_select_clause::<ColumnType>(a, None, None);
        assert_eq!(build, "SELECT  * FROM uploader.[COLUMN_TYPE] WHERE 1 = 1 AND [Int] = 2");
        
        let mut a = SqlParameters::new();
        a.insert("Int".to_string(), SqlValue::Int(2));
        let build = build_known_select_clause::<ColumnType>(a, None, Some(10));
        assert_eq!(build, "SELECT TOP 10 * FROM uploader.[COLUMN_TYPE] WHERE 1 = 1 AND [Int] = 2");
    }

    #[tokio::test]
    async fn check_get_query_result() {
        dotenvy::dotenv().ok();
        let sql = "UPDATE uploader.COLUMN_TYPE SET [SqlType] = 'INT' WHERE [SqlType] = 'INT'".to_string();
        let result = get_query_result(sql).await.unwrap();
        let a = result.rows_affected();
        assert_eq!(a, [1]);
        let b = result.total();
        assert_eq!(b, 1);
    }

    #[tokio::test]
    async fn check_get_query() {
        dotenvy::dotenv().ok();
        let sql = "SELECT * FROM uploader.COLUMN_TYPE".to_string();
        let result = run_query(sql).await.unwrap();
        assert_eq!(result, vec![VAR_TYPES as u64]);
        
        let sql = "UPDATE uploader.COLUMN_TYPE SET [SqlType] = 'INT' WHERE [SqlType] = 'INT'".to_string();
        let result = run_query(sql).await.unwrap();
        assert_eq!(result, vec![1]);
    }

    #[tokio::test]
    async fn check_get_query_response() {
        dotenvy::dotenv().ok();

        let result = get_response_from::<ColumnType>(
            "_test".to_string(), 
            SqlParameters::new()
        ).await.unwrap();
        
        assert_eq!(result.len(), VAR_TYPES);
    }

    #[tokio::test]
    async fn check_get_query_single_response() {
        dotenvy::dotenv().ok();

        let result = get_single_response_from::<i32>(
            "_test".to_string(),
            Some(ColumnType::COL_PK),
        ).await.unwrap();
        
        assert_eq!(result, Some(1));

        let result = get_single_response_from::<i32>(
            "_test".to_string(),
            None,
        ).await.unwrap();
        
        assert_eq!(result, Some(1));
    }

    #[tokio::test]
    async fn check_select_from() {
        dotenvy::dotenv().ok();

        let result = select_from::<ColumnType>(SqlParameters::new(), None, None).await.unwrap();

        assert_eq!(result.len(), VAR_TYPES);
    }

    #[tokio::test]
    async fn check_select_column_from() {
        dotenvy::dotenv().ok();

        let result = select_column_from::<ColumnType, String>(
            ColumnType::COL_SQL_TYPE,
            SqlParameters::new(), 
            None, 
        ).await.unwrap();

        assert_eq!(result.len(), VAR_TYPES);
    }

    #[tokio::test]
    async fn check_select_single() {
        dotenvy::dotenv().ok();
        
        let result = select_single_from::<ColumnType, i32>(SqlParameters::new(), ColumnType::COL_PK).await.unwrap();
        assert_eq!(result, Some(1));
    }

    #[tokio::test]
    async fn check_get_generic_response() {
        dotenvy::dotenv().ok();
        let rows = get_generic_response("SELECT * FROM uploader.[COLUMN_TYPE]".to_string()).await.unwrap();
        
        assert_eq!(rows[0].len(), 3)
    }
    
    #[tokio::test]
    async fn check_select_generic() {
        dotenvy::dotenv().ok();
        let rows = select_generic("COLUMN_TYPE", SqlParameters::new(), None, None).await.unwrap();

        assert_eq!(rows[0].len(), 3)
    }
}