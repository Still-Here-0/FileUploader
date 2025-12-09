use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pk: i32,
    name: String,
    active: bool
}

impl Board {
    pub const COL_PK: &'static str = "pk";
    pub const COL_NAME: &'static str = "Name";
    pub const COL_ACTIVE: &'static str = "Active";
    
    pub fn db_new(pk: i32, name: String, active: bool) -> Self {
        Self { 
            pk,
            name,
            active
        }
    }
}

use super::super::DBLoad;
use super::super::tiberius_interface::FromOwnedSql;

dbload!(Board, "BOARD", COL_PK, COL_NAME, COL_ACTIVE);

// This is a Rust macro that code expands to this code:
//
// impl DBLoad<3> for Board {
//     const TAB: &'static str = "BOARD";
//     const COLS: [&'static str; 3] = [Self::COL_PK, Self::COL_NAME, Self::COL_ACTIVE];

//     fn from_stream(stream: QueryStream<'_>) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<Self>>> + Send + '_>> {
//         let mut row_stream = stream.into_row_stream();
//         let mut result = Vec::new();

//         Box::pin(async move {
//             while let Some(row) = row_stream.next().await.transpose()? as Option<tiberius::Row> {

//                 let pk = row.try_get(Self::COL_PK)?.unwrap();
//                 let sql_type = row.try_get(Self::COL_NAME)?.unwrap();
//                 let view_type = row.try_get(Self::COL_ACTIVE)?.unwrap();

//                 result.push(Self::db_new(pk, sql_type, view_type));
//             }

//             Ok(result)
//         })
//     }
// }
