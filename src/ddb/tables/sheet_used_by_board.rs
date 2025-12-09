use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct SheetUsedByBoard {
    sheet_fk: i32,
    board_fk: i32,
}

impl SheetUsedByBoard {
    pub const COL_SHEET_FK: &'static str = "Sheet_fk";
    pub const COL_BOARD_FK: &'static str = "Board_fk";
    
    pub fn db_new(
        sheet_fk: i32,
        board_fk: i32,
    ) -> Self {
        Self {
            sheet_fk,
            board_fk,
        }
    }
}


use super::super::DBLoad;
use super::super::tiberius_interface::FromOwnedSql;

dbload!(SheetUsedByBoard, "SHEET_USED_BY_BOARD", COL_SHEET_FK, COL_BOARD_FK);
