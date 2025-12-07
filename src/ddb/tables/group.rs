use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pk: i32,
    name: String,
    active: bool,
    board_id: i32,
    last_edited_by_fk: i32
}

impl Group {
    pub const COL_PK: &'static str = "pk";
    pub const COL_NAME: &'static str = "Name";
    pub const COL_ACTIVE: &'static str = "Active";
    pub const COL_BOARD_ID: &'static str = "BoardId";
    pub const COL_LAST_EDITED_BY_FK: &'static str = "LastEditedBy_fk";
    
    pub fn db_new(
        pk: i32,
        name: &str,
        active: bool,
        board_id: i32,
        last_edited_by_fk: i32
    ) -> Self {
        let name = name.to_string();
        
        Self {
            pk,
            name,
            active,
            board_id,
            last_edited_by_fk
        }
    }
}


use super::super::DBLoad;

dbload!(Group, "GROUP", COL_PK, COL_NAME, COL_ACTIVE, COL_BOARD_ID, COL_LAST_EDITED_BY_FK);
