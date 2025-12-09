use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pk: i32,
    active: bool,
    board_fk: Option<i32>,
    worker_fk: i32,
    is_super_user: bool,
}

impl Profile {
    pub const COL_PK: &'static str = "pk";
    pub const COL_ACTIVE: &'static str = "Active";
    pub const COL_BOARD_FK: &'static str = "Board_fk";
    pub const COL_WORKER_FK: &'static str = "Worker_fk";
    pub const COL_IS_SUPER_USER: &'static str = "IsSuperUser";
    
    pub fn db_new(
        pk: i32,
        active: bool,
        board_fk: Option<i32>,
        worker_fk: i32,
        is_super_user: bool,
    ) -> Self {
        Self {
            pk,
            active,
            board_fk,
            worker_fk,
            is_super_user
        }
    }
}


use super::super::DBLoad;
use super::super::tiberius_interface::FromOwnedSql;

dbload!(Profile, "PROFILE", COL_PK, COL_ACTIVE, COL_BOARD_FK?, COL_WORKER_FK, COL_IS_SUPER_USER);
