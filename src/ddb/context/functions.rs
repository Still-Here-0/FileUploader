use std::collections::HashSet;
use std::{env, fs};

use chrono::{NaiveDate, NaiveDateTime};
use futures::StreamExt;
use regex::Regex;
use tiberius::{AuthMethod, Client, Config, ExecuteResult, IntoSql, Query};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use super::super::DBLoad;
use super::db_types::{ChainExec, ChainMap, SqlValue, ToSqlValue, SqlSingleParameters, SqlMultipleParameters};
use super::tiberius_interface::{FromOwnedSql, TiberiusCoversion};
use crate::st;

const GET_IDTT: &str = "SELECT CAST(SCOPE_IDENTITY() AS BIGINT)";

/* #region PRIVATE FUNCTIONS */

fn build_where_clause(map: &SqlSingleParameters) -> String {
    let mut where_clause = String::new();

    for (key, value) in map {
        where_clause.push_str("AND [");
        where_clause.push_str(&key);
        where_clause.push_str("] ");

        where_clause.push_str(&value.tag_sql_where(&key));
    }

    format!("WHERE {}", &where_clause[4..])
}

fn build_set_clause(map: &SqlSingleParameters) -> anyhow::Result<String> {
    anyhow::ensure!(!map.is_empty(), "Update must set at least one column");

    let mut set_clause = String::new();

    for (key, value) in map {
        set_clause.push_str(", [");
        set_clause.push_str(&key);
        set_clause.push_str("] = ");
        set_clause.push_str(&value.tag(&key));
    }

    Ok(format!("SET {}", &set_clause[2..]))
}

fn build_columns_clause(columns: &[String]) -> String {
    format!(
        "({})",
        columns.iter()
            .map(|s| format!("[{s}]"))
            .collect::<Vec<String>>()
            .join(", ")
    )
}

fn build_values_clause(
    insert_parameters: &SqlMultipleParameters,
    header: &[String],
) -> anyhow::Result<String> {
    anyhow::ensure!(!header.is_empty(), "Insert must have at least one column");
    anyhow::ensure!(insert_parameters.len() > 0, "Insert must have at least one column");
    
    let row_count = insert_parameters.hight();
    anyhow::ensure!(row_count > 0, "Insert must have at least one row");

    let mut rows_sql = Vec::with_capacity(row_count);
    for row_idx in 0..row_count {
        let mut values = Vec::with_capacity(header.len());
        for col_name in header {
            let v = insert_parameters.get_value(col_name, row_idx)?;
            values.push(v.tag(&format!("{col_name}_{row_idx}")));
        }
        rows_sql.push(format!("({})", values.join(", ")));
    }

    let rows = rows_sql.join(", ");
    Ok(format!("VALUES {rows};"))
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

    let mut params = Vec::new();
    let mut seen = HashSet::new();

    for m in re.find_iter(sql) {
        let param = m.as_str();

        if seen.insert(param) {
            params.push(param.to_string());
        }
    }

    params
}

fn get_query_text(sql: &str) -> String {
    let path = env::current_dir().unwrap().to_string_lossy().to_string();
    let queries_path = format!("{path}/queries");

    match fs::read_to_string(format!("{queries_path}/{sql}.sql")) {
        Ok(file_sql) => remove_sql_comments(&file_sql),
        Err(_) => remove_sql_comments(sql),
    }
}

fn parse_sql(sql: String, parameters: Option<&SqlSingleParameters>) -> anyhow::Result<(String, Vec<&SqlValue>)> {
    let mut sql_final = get_query_text(&sql);

    let mut sql_parameters = Vec::<&SqlValue>::new();
    if let Some(parameters_map) = parameters {
        let mut connection_parameter_idx = 1;
        let parameters_tags = extract_sql_params(&sql_final);

        for parameter in parameters_tags {
            let mut key = parameter
                .strip_prefix("@_")
                .or_else(|| parameter.strip_prefix("@"))
                .unwrap_or(&parameter);

            let value = parameters_map
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("Parameter '{parameter}' not passed"))?;

            // Parameters that can be inserted in place
            if parameter.starts_with("@_") {
                sql_final = sql_final.replace(&parameter, &value.to_sql());
            }
            // Parameters that must be passed throw the sql interface
            else {
                sql_parameters.push(value);
                sql_final = sql_final.replace(&parameter, &format!("@P{connection_parameter_idx}"));
                connection_parameter_idx += 1;
            }
        }
    }

    Ok((sql_final, sql_parameters))
}

/* #endregion */

/* #region PUBLIC HELP FUNCS */

fn parse_query<'a>(
    sql: String, 
    parameters: Option<&SqlSingleParameters>,
) -> anyhow::Result<Query<'a>> {
    let (sql_final, sql_parameters) = parse_sql(sql, parameters)?;

    let mut final_query = Query::new(sql_final);

    for sql_parameter in sql_parameters {
        let _ = sql_parameter.bind_value(&mut final_query)?;
    }

    Ok(final_query)
}

/* #endregion */

/* #region PUBLIC BUILD SQL */

pub fn build_select_clause(
    table_name: &str,
    where_parameters: Option<&SqlSingleParameters>,
    columns: Option<Vec<&str>>,
    top: Option<u8>,
) -> String {
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

pub fn build_insert_clause(
    table_name: &str,
    insert_parameters: &SqlMultipleParameters,
) -> anyhow::Result<String> {
    let header = insert_parameters.header();
    let columns = build_columns_clause(&header);
    let values = build_values_clause(&insert_parameters, &header)?;

    Ok(format!("INSERT INTO uploader.[{table_name}] {columns} {values}"))
}

pub fn build_delete_clause(
    table_name: &str,
    where_parameters: Option<&SqlSingleParameters>,
) -> String {
    let mut where_clause = st!("");
    if let Some(where_parameters) = where_parameters {
        where_clause = build_where_clause(where_parameters);
    }

    format!("DELETE FROM uploader.[{table_name}] {where_clause}")
}

pub fn build_update_clause(
    table_name: &str,
    new_values: &SqlSingleParameters,
    where_parameters: Option<&SqlSingleParameters>,
) -> anyhow::Result<String> {

    let set_clause = build_set_clause(new_values)?;

    let mut where_clause = st!("");
    if let Some(where_parameters) = where_parameters {
        where_clause = build_where_clause(where_parameters);
    }

    Ok(format!("UPDATE uploader.[{table_name}] {set_clause} {where_clause}"))
}

// CREATE NEW SHEET TABLE DATA

/* #endregion */

/* #region PUBLIC SQL FUNCS */

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

pub async fn get_query_result(
    sql: String,
    sql_parameters: Option<&SqlSingleParameters>,
) -> anyhow::Result<ExecuteResult> {
    let query = parse_query(sql, sql_parameters)?;
    let mut client = mssql_client().await?;
    let result = query.execute(&mut client).await?;

    Ok(result)
}

pub async fn run_query(
    sql: String,
    sql_parameters: Option<&SqlSingleParameters>,
) -> anyhow::Result<Vec<u64>> {
    let result = get_query_result(sql, sql_parameters).await?;

    Ok(result.rows_affected().to_vec())
}

pub async fn get_response_from<T>(
    sql: String,
    sql_parameters: Option<&SqlSingleParameters>,
) -> anyhow::Result<Vec<T>>
where
    T: DBLoad,
{
    let query = parse_query(sql, sql_parameters)?;
    let mut client = mssql_client().await?;
    let mut stream = query.query(&mut client).await?;

    T::from_stream(stream).await
}

pub async fn get_single_response_from<R>(
    sql: String,
    column_name: Option<&str>,
    sql_parameters: Option<&SqlSingleParameters>,
) -> anyhow::Result<Option<R>>
where
    R: TiberiusCoversion,
{
    let mut query = parse_query(sql, sql_parameters)?;    
    let mut client = mssql_client().await?;
    let mut stream = query.query(&mut client).await?;

    let row = stream
        .into_row()
        .await?
        .ok_or(anyhow::anyhow!("No data in the given query"))?;

    let value = 
    match column_name {
        Some(name) => {
            let value = row.try_get_by_name(name)?;
            value
        }
        None => {
            let value = row.try_get_by_index(0)?;
            value
        }
    };

    Ok(value)
}

pub async fn get_identity(
    sql: String,
    sql_parameters: Option<&SqlSingleParameters>,
) -> anyhow::Result<Option<i64>> {
    let sql = format!("{sql} {GET_IDTT}");

    let id = get_single_response_from::<i64>(sql, None, sql_parameters).await?;

    Ok(id)
}

pub async fn select_from<T>(
    where_parameters: Option<&SqlSingleParameters>,
    columns: Option<Vec<&str>>,
    top: Option<u8>,
) -> anyhow::Result<Vec<T>>
where
    T: DBLoad,
{
    let sql = build_select_clause(T::TAB, where_parameters, columns, top);
    get_response_from(sql, None).await
}

pub async fn select_column_from<T, R>(
    column_name: &str,
    where_parameters: Option<&SqlSingleParameters>,
    top: Option<u8>,
) -> anyhow::Result<Vec<Option<R>>>
where
    T: DBLoad,
    R: TiberiusCoversion,
{
    let columns = Some(vec![column_name]);
    let sql = build_select_clause(T::TAB, where_parameters, columns, top);
    let query = parse_query(sql, None)?;

    let mut client = mssql_client().await?;
    let mut stream = query.query(&mut client).await?;
    let mut row_stream = stream.into_row_stream();

    let mut result = Vec::new();
    while let Some(row) = row_stream.next().await.transpose()? {
        let value = row.try_get_by_name(column_name)?;
        result.push(value);
    }

    Ok(result)
}

pub async fn select_single_from<T, R>(
    where_parameters: Option<SqlSingleParameters>,
    column_name: &str,
) -> anyhow::Result<Option<R>>
where
    T: DBLoad,
    R: TiberiusCoversion,
{
    let sql = build_select_clause(T::TAB, None, None, None);
    let query = parse_query(sql, None)?;

    let mut client = mssql_client().await?;
    let mut stream = query.query(&mut client).await?;
    let mut row_stream = stream.into_row_stream();

    let mut result = match row_stream.next().await.transpose()? {
        Some(value) => value.try_get_by_name(column_name)?,
        None => None,
    };

    Ok(result)
}

pub async fn get_generic_response(
    sql: String,
    sql_parameters: Option<&SqlSingleParameters>,
) -> anyhow::Result<Vec<tiberius::Row>> {
    let query = parse_query(sql, sql_parameters)?;
    let mut client = mssql_client().await?;
    let mut stream = query.query(&mut client).await?;
    let mut row_stream = stream.into_row_stream();

    let mut result = Vec::new();

    while let Some(row) = row_stream.next().await.transpose()? as Option<tiberius::Row> {
        result.push(row);
    }

    Ok(result)
}

pub async fn select_generic(
    table_name: &str,
    where_parameters: Option<&SqlSingleParameters>,
    columns: Option<Vec<&str>>,
    top: Option<u8>,
) -> anyhow::Result<Vec<tiberius::Row>> {
    let sql = build_select_clause(table_name, where_parameters, columns, top);

    get_generic_response(sql, None).await
}

pub async fn chain_executions<'a>(
    chain_map: ChainMap<'a>,
    mut global_values: SqlSingleParameters,
) -> anyhow::Result<Vec<u64>> {
    let mut rows_affected = Vec::<u64>::new();

    let mut client = mssql_client().await?;
    client.simple_query(st!("BEGIN TRANSACTION")).await?;
    
    let result = async {
        for (exec, mult, sing) in chain_map {
            let (sql, parameters, new_global) = exec(mult, sing, &mut global_values)?;
            
            match new_global {
                Some(name) => {
                    let sql = format!("{sql} {GET_IDTT}");
                    let query = parse_query(sql.clone(), parameters.as_ref())?;
                    let stream = query.query(&mut client).await?;
                    let row = stream
                        .into_row()
                        .await?
                        .ok_or(anyhow::anyhow!("No data returned from query: {sql}"))?;
                    
                    let identity = row.try_get_by_index::<i64>(0)?
                        .ok_or(anyhow::anyhow!("No identity value found in query: {sql}"))?;
                
                    global_values.insert(name, identity.to_sql_value());
                    rows_affected.push(1);
                },
                None => {
                    let query = parse_query(sql, parameters.as_ref())?;
                    let partial_result = query.execute(&mut client).await?;
                    rows_affected.extend(partial_result.rows_affected());
                }
            }
        }
        
        Ok::<Vec<u64>, anyhow::Error>(rows_affected)
    };

    match result.await {
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
    use futures::TryStreamExt;
    use tiberius::ToSql;

    use super::*;
    use crate::ddb::{
        DBLoad,
        context::db_types::{ChainReturn, SqlValue},
        tables::*,
    };

    fn mult_parameters() -> SqlMultipleParameters {
        let mut a = SqlMultipleParameters::new();

        for i in 0..3 {
            a.add_line(
                vec![
                    (ColumnType::COL_SQL_TYPE, SqlValue::Str(format!("TEST {i}"))),
                    (ColumnType::COL_VIEW_TYPE, SqlValue::Int(i)),
                ]
            ).unwrap();
        }

        a
    }

    /* #region PRIVATE FUNCTIONS */

    #[test]
    fn check_build_set_clause() {
        let mut new_values = SqlSingleParameters::new();
        new_values.insert(
            st!(ColumnType::COL_SQL_TYPE),
            SqlValue::Str(st!("TEST")),
        );

        let set_clause = build_set_clause(&new_values).unwrap();
        assert_eq!(set_clause, st!("SET [SqlType] = @_SqlType"));
    }

    #[test]
    fn check_build_set_clause_empty_is_err() {
        let new_values = SqlSingleParameters::new();
        assert!(build_set_clause(&new_values).is_err());
    }

    #[test]
    fn check_build_columns_clause() {
        let columns = vec![st!(ColumnType::COL_SQL_TYPE), st!(ColumnType::COL_VIEW_TYPE)];
        let clause = build_columns_clause(&columns);
        assert_eq!(clause, st!("([SqlType], [ViewType])"));
    }

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
            WHERE id = @_user_id
            AND status = @status
            AND owner = @_user_id
        ";

        let params = extract_sql_params(sql);

        assert!(params.contains(&st!("@_user_id")));
        assert!(params.contains(&st!("@status")));
    }

    #[test]
    fn check_parse_sql() {
        let sql = "SELECT * FROM users /* aaa */ WHERE id = @_user_id AND status IN @_status AND bin = @BIN-- AAAAAAAAA";
        let mut sql_parameters = SqlSingleParameters::new();
        sql_parameters.insert(st!("user_id"), SqlValue::Int(123456));
        sql_parameters.insert(st!("status"), vec![st!("ON"), st!("OFF")].to_sql_value());
        sql_parameters.insert(st!("BIN"), st!("0x...").to_sql_value());

        let output = "SELECT * FROM users  WHERE id = 123456 AND status IN ('ON', 'OFF') AND bin = @P1";

        let (new_sql, parameters) = parse_sql(st!(sql), Some(&sql_parameters)).unwrap();

        assert_eq!(new_sql, output);
        assert_eq!(parameters.len(), 1);
        assert_eq!(parameters[0].to_string(), st!("0x..."));
    }

    #[test]
    fn check_build_where() {
        let mut a = SqlSingleParameters::new();
        a.insert(st!("Int"), SqlValue::Int(2));
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [Int] = @_Int"));

        let mut a = SqlSingleParameters::new();
        a.insert(st!("Float"), SqlValue::Float(2.5));
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [Float] = @_Float"));

        let mut a = SqlSingleParameters::new();
        a.insert(st!("Bool"), SqlValue::Bool(true));
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [Bool] = @_Bool"));

        let mut a = SqlSingleParameters::new();
        a.insert(st!("StringL"), SqlValue::StrL(st!("OLOKO")));
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [StringL] LIKE @_StringL"));

        let mut a = SqlSingleParameters::new();
        a.insert(st!("StringN"), SqlValue::Str(st!(";-;")));
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [StringN] = @_StringN"));

        let mut a = SqlSingleParameters::new();
        a.insert(
            st!("Date"),
            SqlValue::Date(NaiveDate::parse_from_str("2025-12-21", "%Y-%m-%d").unwrap()),
        );
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [Date] = @_Date"));

        let mut a = SqlSingleParameters::new();
        a.insert(
            st!("DateTime"),
            SqlValue::DateTime(
                NaiveDateTime::parse_from_str("2025-12-21 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
        );
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [DateTime] = @_DateTime"));

        let mut a = SqlSingleParameters::new();
        a.insert(st!("IntVec"), SqlValue::IntList(vec![1, 2, 3]));
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [IntVec] IN (@_IntVec)"));

        let mut a = SqlSingleParameters::new();
        a.insert(st!("FloatVec"), SqlValue::FloatList(vec![1.5, 2.5, 3.5]));
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [FloatVec] IN (@_FloatVec)"));

        let mut a = SqlSingleParameters::new();
        a.insert(
            st!("StrVec"),
            SqlValue::StrList(vec![st!("1"), st!("2"), st!("3")]),
        );
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [StrVec] IN (@_StrVec)"));

        let mut a = SqlSingleParameters::new();
        a.insert(
            st!("BIN"),
            SqlValue::Bin(vec![1, 2, 3]),
        );
        let where_c = build_where_clause(&a);
        assert_eq!(where_c, st!("WHERE [BIN] = @BIN"));
    }

    #[test]
    fn check_build_values() {
        let a = mult_parameters();

        let header = vec![st!(ColumnType::COL_SQL_TYPE), st!(ColumnType::COL_VIEW_TYPE)];
        let result = build_values_clause(&a, &header).unwrap();

        assert_eq!(
            result,
            st!("VALUES (@_SqlType_0, @_ViewType_0), (@_SqlType_1, @_ViewType_1), (@_SqlType_2, @_ViewType_2);")
        );
    }

    #[test]
    fn check_build_delete_clause() {
        let sql = build_delete_clause(ColumnType::TAB, None);
        assert_eq!(sql, st!("DELETE FROM uploader.[COLUMN_TYPE] "));

        let mut where_parameters = SqlSingleParameters::new();
        where_parameters.insert(st!(ColumnType::COL_PK), SqlValue::Int(1));
        let sql = build_delete_clause(ColumnType::TAB, Some(&where_parameters));
        assert_eq!(sql, st!("DELETE FROM uploader.[COLUMN_TYPE] WHERE [pk] = @_pk"));
    }

    /* #endregion */

    /* #region PUBLIC BUILD SQL */

    #[test]
    fn check_build_select() {
        let build = build_select_clause(ColumnType::TAB, None, None, None);
        assert_eq!(build, st!("SELECT  * FROM uploader.[COLUMN_TYPE] "));

        let build = build_select_clause(ColumnType::TAB, None, None, Some(10));
        assert_eq!(build, st!("SELECT TOP 10 * FROM uploader.[COLUMN_TYPE] "));

        let mut a = SqlSingleParameters::new();
        a.insert(st!("Int"), SqlValue::Int(2));
        let build = build_select_clause(ColumnType::TAB, Some(&a), None, None);
        assert_eq!(
            build,
            st!("SELECT  * FROM uploader.[COLUMN_TYPE] WHERE [Int] = @_Int")
        );

        let mut a = SqlSingleParameters::new();
        a.insert(st!("Int"), SqlValue::Int(2));
        let build = build_select_clause(ColumnType::TAB, Some(&a), None, Some(10));
        assert_eq!(
            build,
            st!("SELECT TOP 10 * FROM uploader.[COLUMN_TYPE] WHERE [Int] = @_Int")
        );
    }

    #[test]
    fn check_build_insert() {
        let a = mult_parameters();
        let sql = build_insert_clause(ColumnType::TAB, &a).unwrap();

        // SqlMultParameters::header() is HashMap-based, so column order is not guaranteed.
        // But values are generated using the same header, so the statement should match
        // one of these two possible orderings.
        let expected_a =
            "INSERT INTO uploader.[COLUMN_TYPE] ([SqlType], [ViewType]) VALUES (@_SqlType_0, @_ViewType_0), (@_SqlType_1, @_ViewType_1), (@_SqlType_2, @_ViewType_2);";
        let expected_b =
            "INSERT INTO uploader.[COLUMN_TYPE] ([ViewType], [SqlType]) VALUES (@_ViewType_0, @_SqlType_0), (@_ViewType_1, @_SqlType_1), (@_ViewType_2, @_SqlType_2);";
        assert!(sql == expected_a || sql == expected_b);
    }

    #[test]
    fn check_build_update_clause() {
        let mut new_values = SqlSingleParameters::new();
        new_values.insert(
            st!(ColumnType::COL_SQL_TYPE),
            SqlValue::Str(st!("TEST")),
        );

        let mut where_parameters = SqlSingleParameters::new();
        where_parameters.insert(st!(ColumnType::COL_PK), SqlValue::Int(1));

        let sql = build_update_clause(ColumnType::TAB, &new_values, Some(&where_parameters)).unwrap();
        assert_eq!(
            sql,
            st!("UPDATE uploader.[COLUMN_TYPE] SET [SqlType] = @_SqlType WHERE [pk] = @_pk")
        );
    }

    /* #endregion */

    /* #region PUBLIC FUNCITONS */

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

        let result = get_response_from::<ColumnType>(st!("_test"), None)
            .await
            .unwrap();

        assert!(result.len() > 0);
    }

    #[tokio::test]
    async fn check_get_query_single_response() {
        dotenvy::dotenv().ok();

        let result =
            get_single_response_from::<String>(st!("_test"), Some(ColumnType::COL_SQL_TYPE), None)
                .await
                .unwrap();

        assert!(result.is_some());

        let result = get_single_response_from::<i32>(st!("_test"), None, None)
            .await
            .unwrap();

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

        let result = select_column_from::<ColumnType, String>(ColumnType::COL_SQL_TYPE, None, None)
            .await
            .unwrap();

        assert!(result.len() > 0);
    }

    #[tokio::test]
    async fn check_select_single() {
        dotenvy::dotenv().ok();

        let result = select_single_from::<ColumnType, i32>(None, ColumnType::COL_PK)
            .await
            .unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn check_get_generic_response() {
        dotenvy::dotenv().ok();
        let rows = get_generic_response(st!("SELECT * FROM uploader.[COLUMN_TYPE]"), None)
            .await
            .unwrap();

        assert!(rows.len() > 0);
        assert_eq!(rows[0].len(), 3);
    }

    #[tokio::test]
    async fn check_select_generic() {
        dotenvy::dotenv().ok();
        let rows = select_generic("COLUMN_TYPE", None, None, None)
            .await
            .unwrap();

        assert!(rows.len() > 0);
        assert_eq!(rows[0].len(), 3);
    }

    #[tokio::test]
    async fn check_chain_execution() {
        fn do_nothing(
            mult: Option<SqlMultipleParameters>,
            sing: Option<SqlSingleParameters>, 
            glob: &SqlSingleParameters
        ) -> ChainReturn {
            Ok((st!(""), None, None))
        }

        fn init_insert(
            mult: Option<SqlMultipleParameters>,
            sing: Option<SqlSingleParameters>, 
            glob: &SqlSingleParameters
        ) -> ChainReturn {
            let sql = st!("INSERT INTO uploader.COLUMN_TYPE (SqlType, ViewType) VALUES ('asdaf', 'sadfas');");
            Ok((sql, None, Some(st!("pk"))))
        }

        fn delete_init(
            mult: Option<SqlMultipleParameters>,
            sing: Option<SqlSingleParameters>, 
            glob: &SqlSingleParameters
        ) -> ChainReturn {
            let mut parameter = SqlSingleParameters::new();
            parameter.insert(
                st!("pk"), 
                glob.get(&st!("pk"))
                    .ok_or(anyhow::anyhow!("No 'pk' value found"))?
                    .clone()
            );
            let sql = st!("DELETE FROM uploader.COLUMN_TYPE WHERE pk = @_pk");

            Ok((sql, Some(parameter), None))
        }

        fn next_insert(
            mult: Option<SqlMultipleParameters>,
            sing: Option<SqlSingleParameters>, 
            glob: &SqlSingleParameters
        ) -> ChainReturn {
            let mut parameter = SqlSingleParameters::new();
            parameter.insert(
                st!("pk"), 
                glob.get(&st!("pk"))
                    .ok_or(anyhow::anyhow!("No 'pk' value found"))?
                    .clone()
            );

            let sql = format!("INSERT INTO uploader.COLUMN_TYPE (SqlType, ViewType) VALUES (@_pk, @_pk);");
            
            Ok((sql, Some(parameter), Some(st!("pk"))))
        }

        fn error_insert<'a>(
            mult: Option<SqlMultipleParameters>,
            sing: Option<SqlSingleParameters>, 
            glob: &SqlSingleParameters
        ) -> ChainReturn {
            Ok((format!("||ERROR||"), None, None))
        }

        dotenvy::dotenv().ok();
        let mut chain_exec = ChainMap::new();
        chain_exec.push(&init_insert, None, None);
        chain_exec.push(&delete_init, None, None);
        let mut p = SqlSingleParameters::new();
        let v = chain_executions(chain_exec, p).await;
        assert!(v.is_ok());

        let mut chain_exec = ChainMap::new();
        chain_exec.push(&init_insert, None, None);
        chain_exec.push(&next_insert, None, None);
        chain_exec.push(&next_insert, None, None);
        chain_exec.push(&error_insert, None, None);
        let mut p = SqlSingleParameters::new();

        let v = chain_executions(chain_exec, p).await;
        assert!(v.is_err());
        let err_msg = v.unwrap_err().to_string();
        assert!(err_msg.contains("Incorrect syntax near '|'"));

        let mut chain_exec = ChainMap::new();
        chain_exec.push(&do_nothing, None, None);
        chain_exec.push(&do_nothing, None, None);
        chain_exec.push(&do_nothing, None, None);
        chain_exec.push(&do_nothing, None, None);
        let mut p = SqlSingleParameters::new();

        let v = chain_executions(chain_exec, p).await;

        assert!(v.is_ok())
    }

    /* #endregion */
}
