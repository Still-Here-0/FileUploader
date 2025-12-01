use std::{pin::Pin, result};

use futures::{Stream, StreamExt};
use tiberius::QueryStream;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnType {
    pk: i32,
    sql_type: String,
    view_type: String
}

type A = std::pin::Pin<Box<dyn Stream<Item = Result<tiberius::Row, tiberius::error::Error>> + Send>>;

impl ColumnType {
    pub const COL_PK: &'static str = "pk";
    pub const COL_SQL_TYPE: &'static str = "SqlType";
    pub const COL_VIEW_TYPE: &'static str = "ViewType";

    pub fn new(pk: i32, sql_type: &str, view_type: &str) -> Self {
        let sql_type = sql_type.to_string();
        let view_type = view_type.to_string();
        
        ColumnType { 
            pk, 
            sql_type, 
            view_type
        }
    }
}

impl super::DBLoad for ColumnType {
    fn from_row_stream(stream: QueryStream<'_>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Self>>> + Send + '_>> {
        let mut row_stream = stream.into_row_stream();
        
        Box::pin(async move {
            let mut result = Vec::new();
            while let Some(row) = row_stream.next().await {
                let row = row?;
    
                let pk = row.get::<i32, _>(Self::COL_PK).unwrap();
                let sql_type = row.get::<&str, _>(Self::COL_SQL_TYPE).unwrap();
                let view_type = row.get::<&str, _>(Self::COL_VIEW_TYPE).unwrap();
    
                result.push(ColumnType::new(pk, sql_type, view_type));
            }
    
            Ok(result)
        })
    }
}