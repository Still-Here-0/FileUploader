use std::collections::HashSet;
use std::{env, fs};

use chrono::{NaiveDate, NaiveDateTime};
use futures::StreamExt;
use regex::Regex;
use tiberius::{AuthMethod, Client, Config, ExecuteResult, Query};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use super::db_types::{SqlParameters, SqlValue, ChainExec};
use super::super::{DBLoad};
use super::tiberius_interface::{TiberiusCoversion, FromOwnedSql};
use crate::st;

/* #region PRIVATE FUNCTIONS */

fn build_where_clause(map: SqlParameters) -> String {
    let mut where_clause = String::new();

    for (key, value) in map {
        where_clause.push_str("AND [");
        where_clause.push_str(&key);
        where_clause.push_str("] ");

        where_clause.push_str(&value.to_sql_where());
    }

    where_clause = st!(&where_clause[4..]);
    format!("WHERE {where_clause}")
}

fn build_known_select_clause<T>(where_parameters: Option<SqlParameters>, columns: Option<Vec<&str>>, top: Option<u8>) -> String
    where 
        T: DBLoad
{
    let table_name = T::TAB;
    
    build_generic_select_clause(table_name, where_parameters, columns, top)
}

fn build_generic_select_clause(table_name: &str, where_parameters: Option<SqlParameters>, columns: Option<Vec<&str>>, top: Option<u8>) -> String {
    
    let mut where_clause = st!("");
    if let Some(where_parameters) = where_parameters {
        where_clause = build_where_clause(where_parameters);
    }

    let top = match top {
        Some(value) => format!("TOP {value}"),
        None => String::new(),
    };

    let columns = match columns {
        Some(values) => values.join(", "),
        None => st!("*"),
    };
    
    format!("SELECT {top} {columns} FROM uploader.[{table_name}] {where_clause}")
}

fn remove_sql_comments(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let mut chars = sql.chars().peekable();

    let mut in_string = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while let Some(c) = chars.next() {
        // Start line comment
        if !in_string && !in_block_comment && c == '-' && chars.peek() == Some(&'-') {
            chars.next();
            in_line_comment = true;
            continue;
        }

        // Start block comment
        if !in_string && !in_line_comment && c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_block_comment = true;
            continue;
        }

        // End line comment
        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
                result.push('\n');
            }
            continue;
        }

        // End block comment
        if in_block_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block_comment = false;
            }
            continue;
        }

        // Toggle string literal (SQL standard single quotes)
        if c == '\'' {
            in_string = !in_string;
            result.push(c);
            continue;
        }

        // Normal character
        result.push(c);
    }

    result
}

fn extract_sql_params(sql: &str) -> Vec<String> {
    let re = Regex::new(r"@[A-Za-z_][A-Za-z0-9_]*").unwrap();

    let params: HashSet<String> = re
        .find_iter(sql)
        .map(|m| st!(m.as_str()))
        .collect();

    params.into_iter().collect()
}

fn parse_sql(sql: String, sql_parameters: Option<&SqlParameters>) -> String {
    let path = env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let queries_path = format!("{path}/queries");

    let mut sql_final = match fs::read_to_string(format!("{queries_path}/{sql}.sql")) {
        Ok(file_sql) => remove_sql_comments(&file_sql),
        Err(_) => remove_sql_comments(&sql),
    };

    if let Some(parameters_map) = sql_parameters {
        let parameters = extract_sql_params(&sql_final);

        for parameter in parameters {
            let value = parameters_map.get(&parameter)
                .expect(&format!("Parameter '{parameter}' not passed"))
                .to_sql();
            sql_final = sql_final.replace(&parameter, &value)
        }
    }

    sql_final
}

fn parse_query<'a>(sql: String, sql_parameters: Option<&SqlParameters>) -> Query<'a> {
    let sql_final = parse_sql(sql, sql_parameters);

    Query::new(sql_final)
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

pub async fn get_query_result(sql: String, sql_parameters: Option<&SqlParameters>) -> anyhow::Result<ExecuteResult> {
    let query = parse_query(sql, sql_parameters);
    let mut client = mssql_client().await?;
    let result = query.execute(&mut client).await?;

    Ok(result)
}

pub async fn run_query(sql: String, sql_parameters: Option<&SqlParameters>) -> anyhow::Result<Vec<u64>> {
    let result = get_query_result(sql, sql_parameters).await?;
    
    Ok(result.rows_affected().to_vec())
}

pub async fn get_response_from<T>(sql: String, sql_parameters: Option<&SqlParameters>) -> anyhow::Result<Vec<T>> 
    where 
        T: DBLoad,
{
    let query = parse_query(sql, sql_parameters);
    let mut client = mssql_client().await?;
    let mut stream = query.query(&mut client).await?;
    
    T::from_stream(stream).await
}

pub async fn get_single_response_from<R>(sql: String, column_name: Option<&str>, sql_parameters: Option<&SqlParameters>) -> anyhow::Result<Option<R>>
    where 
        R: TiberiusCoversion
{
    let query = parse_query(sql, sql_parameters);
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

pub async fn select_from<T>(where_parameters: Option<SqlParameters>, columns: Option<Vec<&str>>, top: Option<u8>) -> anyhow::Result<Vec<T>> 
    where 
        T: DBLoad
{
    let sql = build_known_select_clause::<T>(where_parameters, columns, top);
    get_response_from(sql, None).await
}

pub async fn select_column_from<T, R>(column_name: &str, where_parameters: Option<SqlParameters>, top: Option<u8>) -> anyhow::Result<Vec<Option<R>>>
    where 
        T: DBLoad,
        R: TiberiusCoversion
{
    let columns = Some(vec![column_name]);
    let sql = build_known_select_clause::<T>(where_parameters, columns, top);
    let query = parse_query(sql, None);

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

pub async fn select_single_from<T, R>(where_parameters: Option<SqlParameters>, column_name: &str) -> anyhow::Result<Option<R>>
    where 
        T: DBLoad,
        R: TiberiusCoversion
{
    let sql = build_known_select_clause::<T>(where_parameters, None, None);
    let query = parse_query(sql, None);

    let mut client = mssql_client().await?;
    let mut stream= query.query(&mut client).await?;
    let mut row_stream = stream.into_row_stream();
    
    let mut result = match row_stream.next().await.transpose()? {
        Some(value) => value.try_get_by_name(column_name)?,
        None => None,
    };

    Ok(result)
}

pub async fn get_generic_response(sql: String, sql_parameters: Option<&SqlParameters>) -> anyhow::Result<Vec<tiberius::Row>> {
    let query = parse_query(sql, sql_parameters);
    let mut client = mssql_client().await?;
    let mut stream = query.query(&mut client).await?;
    let mut row_stream = stream.into_row_stream();

    let mut result = Vec::new();

    while let Some(row) = row_stream.next().await.transpose()? as Option<tiberius::Row> {
        result.push(row);

    }

    Ok(result)
}

pub async fn select_generic(table_name: &str, where_parameters: Option<SqlParameters>, columns: Option<Vec<&str>>, top: Option<u8>) -> anyhow::Result<Vec<tiberius::Row>> {
    let sql = build_generic_select_clause(table_name, where_parameters, columns, top);

    get_generic_response(sql, None).await
}

async fn chain_executions(chain_exec: ChainExec<'_>, sql_parameters: &mut SqlParameters) -> anyhow::Result<Vec<u64>> {
    let mut rows_affected = Vec::<u64>::new();

    let mut  client = mssql_client().await?;
    client.simple_query(st!("BEGIN TRANSACTION")).await?;

    let result = (|| async {
        for exec in chain_exec {
            let (sql, new_parameter_name) = exec(&sql_parameters);
            let query = parse_query(sql, Some(&sql_parameters));
    
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
                            sql_parameters.insert(name.clone(), SqlValue::Int(value));
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
            client.simple_query(st!("COMMIT")).await?;
            Ok(affected)
        }
        Err(e) => {
            client.simple_query(st!("ROLLBACK")).await?;
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

    #[test]
    fn check_remove_sql_comments() {
        let sql = r#"
SELECT 'this -- is not a comment'
FROM users -- remove this
WHERE id = 10
/* block
comment */
AND name = '/* not a comment */'
        "#;
        
        let output = r#"
SELECT 'this -- is not a comment'
FROM users 
WHERE id = 10

AND name = '/* not a comment */'
        "#;

        let new_sql = remove_sql_comments(sql);
        
        assert_eq!(new_sql, output)
    }

    #[test]
    fn check_extract_sql_params() {
        let sql = "
            SELECT *
            FROM users
            WHERE id = @user_id
            AND status = @status
            AND owner = @user_id
        ";

        let params = extract_sql_params(sql);
        
        assert!(params.contains(&st!("@user_id")));
        assert!(params.contains(&st!("@status")));
    }

    #[test]
    fn check_parse_sql() {
        let sql = "SELECT * FROM users /* aaa */ WHERE id = @user_id AND status IN @status";
        let mut sql_parameters = SqlParameters::new();
        sql_parameters.insert(st!("@user_id"), SqlValue::Int(123456));
        sql_parameters.insert(st!("@status"), SqlValue::StrList(vec![st!("ON"), st!("OFF")]));

        let output = "SELECT * FROM users  WHERE id = 123456 AND status IN ('ON', 'OFF')";

        let new_sql = parse_sql(st!(sql), Some(&sql_parameters));

        assert_eq!(new_sql, output)
    }

    #[test]
    fn check_build_where() {
        let mut a = SqlParameters::new();
        a.insert(st!("Int"), SqlValue::Int(2));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [Int] = 2"));

        let mut a = SqlParameters::new();
        a.insert(st!("Float"), SqlValue::Float(2.5));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [Float] = 2.5"));

        let mut a = SqlParameters::new();
        a.insert(st!("Bool"), SqlValue::Bool(true));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [Bool] = 1"));

        let mut a = SqlParameters::new();
        a.insert(st!("StringL"), SqlValue::Str(st!("OLOKO"), true));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [StringL] like 'OLOKO'"));
        
        let mut a = SqlParameters::new();
        a.insert(st!("StringN"), SqlValue::Str(st!(";-;"), false));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [StringN] = '-'"));

        let mut a = SqlParameters::new();
        a.insert(st!("Date"), SqlValue::Date(NaiveDate::parse_from_str("2025-12-21", "%Y-%m-%d").unwrap()));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [Date] = '2025-12-21'"));

        let mut a = SqlParameters::new();
        a.insert(st!("DateTime"), SqlValue::DateTime(NaiveDateTime::parse_from_str("2025-12-21 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap()));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [DateTime] = '2025-12-21 10:30:00'"));
        
        let mut a = SqlParameters::new();
        a.insert(st!("IntVec"), SqlValue::IntList(vec![1, 2, 3]));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [IntVec] IN (1, 2, 3)"));

        let mut a = SqlParameters::new();
        a.insert(st!("FloatVec"), SqlValue::FloatList(vec![1.5, 2.5, 3.5]));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [FloatVec] IN (1.5, 2.5, 3.5)"));

        let mut a = SqlParameters::new();
        a.insert(st!("StrVec"), SqlValue::StrList(vec![st!("1"), st!("2"), st!("3")]));
        let where_c = build_where_clause(a);
        assert_eq!(where_c, st!("WHERE [StrVec] IN ('1', '2', '3')"));
    }

    #[test]
    fn check_build_select() {
        let build = build_known_select_clause::<ColumnType>(None, None, None);
        assert_eq!(build, st!("SELECT  * FROM uploader.[COLUMN_TYPE] "));
        
        let build = build_known_select_clause::<ColumnType>(None, None, Some(10));
        assert_eq!(build, st!("SELECT TOP 10 * FROM uploader.[COLUMN_TYPE] "));
        
        let mut a = SqlParameters::new();
        a.insert(st!("Int"), SqlValue::Int(2));
        let build = build_known_select_clause::<ColumnType>(Some(a), None, None);
        assert_eq!(build, st!("SELECT  * FROM uploader.[COLUMN_TYPE] WHERE [Int] = 2"));
        
        let mut a = SqlParameters::new();
        a.insert(st!("Int"), SqlValue::Int(2));
        let build = build_known_select_clause::<ColumnType>(Some(a), None, Some(10));
        assert_eq!(build, st!("SELECT TOP 10 * FROM uploader.[COLUMN_TYPE] WHERE [Int] = 2"));
    }

    #[tokio::test]
    async fn check_get_query_result() {
        dotenvy::dotenv().ok();
        let sql = st!("UPDATE uploader.COLUMN_TYPE SET [SqlType] = 'INT' WHERE [SqlType] = 'INT'");
        let result = get_query_result(sql, None).await.unwrap();
        let a = result.rows_affected();

        // Assuming that INT if a default type (every will use it)
        assert_eq!(a, [1]);
        let b = result.total();
        assert_eq!(b, 1);
    }

    #[tokio::test]
    async fn check_get_query() {
        dotenvy::dotenv().ok();
        let sql = st!("SELECT * FROM uploader.COLUMN_TYPE");
        let result = run_query(sql, None).await.unwrap();
        assert!(result.len() > 0);
        
        let sql = st!("UPDATE uploader.COLUMN_TYPE SET [SqlType] = 'INT' WHERE [SqlType] = 'INT'");
        let result = run_query(sql, None).await.unwrap();

        // Assuming that INT if a default type (every will use it)
        assert_eq!(result, vec![1]);
    }

    #[tokio::test]
    async fn check_get_query_response() {
        dotenvy::dotenv().ok();

        let result = get_response_from::<ColumnType>(
            st!("_test"), 
            None
        ).await.unwrap();
        
        assert!(result.len() > 0);
    }

    #[tokio::test]
    async fn check_get_query_single_response() {
        dotenvy::dotenv().ok();

        let result = get_single_response_from::<String>(
            st!("_test"),
            Some(ColumnType::COL_SQL_TYPE),
            None
        ).await.unwrap();

        assert!(result.is_some());

        let result = get_single_response_from::<i32>(
            st!("_test"),
            None,
            None
        ).await.unwrap();
        
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn check_select_from() {
        dotenvy::dotenv().ok();

        let result = select_from::<ColumnType>(None, None, None).await.unwrap();

        assert!(result.len() > 0);
    }

    #[tokio::test]
    async fn check_select_column_from() {
        dotenvy::dotenv().ok();

        let result = select_column_from::<ColumnType, String>(
            ColumnType::COL_SQL_TYPE,
            None, 
            None, 
        ).await.unwrap();

        assert!(result.len() > 0);
    }

    #[tokio::test]
    async fn check_select_single() {
        dotenvy::dotenv().ok();
        
        let result = select_single_from::<ColumnType, i32>(None, ColumnType::COL_PK).await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn check_get_generic_response() {
        dotenvy::dotenv().ok();
        let rows = get_generic_response(st!("SELECT * FROM uploader.[COLUMN_TYPE]"), None).await.unwrap();
        
        assert!(rows.len() > 0);
        assert_eq!(rows[0].len(), 3);
    }
    
    #[tokio::test]
    async fn check_select_generic() {
        dotenvy::dotenv().ok();
        let rows = select_generic("COLUMN_TYPE", None, None, None).await.unwrap();

        assert!(rows.len() > 0);
        assert_eq!(rows[0].len(), 3);
    }

    #[tokio::test]
    async fn check_chain_execution() {
        fn do_nothing(parameters: &SqlParameters) -> ChainReturn {
            (
                st!(""),
                None
            )
        }

        fn init_insert(parameters: &SqlParameters) -> ChainReturn {
            (
                st!("INSERT INTO uploader.COLUMN_TYPE (SqlType, ViewType) OUTPUT INSERTED.pk VALUES ('asdaf', 'sadfas')"),
                Some(st!("pk"))
            )
        }
        
        fn next_insert(parameters: &SqlParameters) -> ChainReturn {
            let pk = parameters.get(&st!("pk")).unwrap().to_string();
            (
                format!("INSERT INTO uploader.COLUMN_TYPE (SqlType, ViewType) OUTPUT INSERTED.pk VALUES ('{pk}', '{pk}')"),
                Some(st!("pk"))
            )
        }

        fn error_insert(parameters: &SqlParameters) -> ChainReturn {
            (
                st!("||ERROR||"),
                None
            )
        }
        
        dotenvy::dotenv().ok();
        let mut chain_exec = ChainExec::new();
        chain_exec.push(&init_insert);
        chain_exec.push(&next_insert);
        chain_exec.push(&next_insert);
        chain_exec.push(&error_insert);
        let mut p = SqlParameters::new();

        let mut  client = mssql_client().await.unwrap();

        let v = chain_executions(chain_exec, &mut p).await;

        assert!(v.is_err());

        let mut chain_exec = ChainExec::new();
        chain_exec.push(&do_nothing);
        chain_exec.push(&do_nothing);
        chain_exec.push(&do_nothing);
        chain_exec.push(&do_nothing);
        let mut p = SqlParameters::new();

        let mut  client = mssql_client().await.unwrap();

        let v = chain_executions(chain_exec, &mut p).await;

        assert!(v.is_ok())
    }

}