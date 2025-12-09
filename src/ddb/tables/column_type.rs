use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnType {
    pk: i32,
    sql_type: String,
    view_type: String
}

impl ColumnType {
    pub const COL_PK: &'static str = "pk";
    pub const COL_SQL_TYPE: &'static str = "SqlType";
    pub const COL_VIEW_TYPE: &'static str = "ViewType";
    
    pub fn db_new(pk: i32, sql_type: String, view_type: String) -> Self {
        Self {
            pk,
            sql_type,
            view_type
        }
    }
}

use super::super::DBLoad;
use super::super::tiberius_interface::FromOwenedSql;

dbload!(ColumnType, "COLUMN_TYPE", COL_PK, COL_SQL_TYPE, COL_VIEW_TYPE);

// This is a Rust macro that code expands to this code:
//
// impl DBLoad<3> for ColumnType {
//     const TAB: &'static str = "COLUMN_TYPE";
//     const COLS: [&'static str; 3] = [Self::COL_PK, Self::COL_SQL_TYPE, Self::COL_VIEW_TYPE];
//
//     fn from_stream(stream: QueryStream<'_>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Self>>> + Send + '_>> {
//
//         let mut row_stream = stream.into_row_stream();
//         let mut result = Vec::new();
//
//         Box::pin(async move {
//             while let Some(row) = row_stream.next().await.transpose()? as Option<tiberius::Row> {
//
//                 let pk = row.get(Self::COL_PK).unwrap();
//                 let sql_type = row.get(Self::COL_SQL_TYPE).unwrap();
//                 let view_type = row.get(Self::COL_VIEW_TYPE).unwrap();
//
//                 result.push(Self::new(pk, sql_type, view_type));
//             }
//
//             Ok(result)
//         })
//     }
// }
