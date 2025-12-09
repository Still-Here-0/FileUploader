use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomSqlScript {
    sheet_fk: i32,
    run_before_update: bool,
    run_after_update: bool,
    run_as_update: bool,
    custom_script: String
}

impl CustomSqlScript {
    pub const COL_SHEET_FK: &'static str = "Sheet_fk";
    pub const COL_RUNBF: &'static str = "RunBeforeUpdate";
    pub const COL_RUNAF: &'static str = "RunAfterUpdate";
    pub const COL_RUNAS: &'static str = "RunAsUpdate";
    pub const COL_SCRIPT: &'static str = "CustomScript";
    
    pub fn db_new(
        sheet_fk: i32, 
        run_before_update: bool,
        run_after_update: bool,
        run_as_update: bool,
        custom_script: String
    ) -> Self {
        Self {
            sheet_fk,
            run_before_update,
            run_after_update,
            run_as_update,
            custom_script
        }
    }
}

use super::super::DBLoad;
use super::super::tiberius_interface::FromOwnedSql;

dbload!(CustomSqlScript, "CUSTOM_SQL_SCRIPT", COL_SHEET_FK, COL_RUNBF, COL_RUNAF, COL_RUNAS, COL_SCRIPT);
