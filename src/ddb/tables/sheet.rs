use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sheet {
    pk: i32,
    description: String,
    table_name: String,
    last_edited_by_fk: i32,
    active: bool,
    days_to_refresh: Option<i32>,
    model: Option<Vec<u8>>,
    request_after_update: Option<String>
}

impl Sheet {
    pub const COL_PK: &'static str = "pk";
    pub const COL_DESCRIPTION: &'static str = "Description";
    pub const COL_TABLE_NAME: &'static str = "TableName";
    pub const COL_LAST_EDITED_BY_FK: &'static str = "LastEditedBy_fk";
    pub const COL_ACTIVE: &'static str = "Active";
    pub const COL_DAYS_TO_REFRESH: &'static str = "DaysToRefresh";
    pub const COL_MODEL: &'static str = "Model";
    pub const COL_REQUEST_AFTER_UPDATE: &'static str = "RequestAfterUpdate";
    
    pub fn db_new(
        pk: i32,
        description: String,
        table_name: String,
        last_edited_by_fk: i32,
        active: bool,
        days_to_refresh: Option<i32>,
        model: Option<Vec<u8>>,
        request_after_update: Option<String>
    ) -> Self {
        Self {
            pk,
            description,
            table_name,
            last_edited_by_fk,
            active,
            days_to_refresh,
            model,
            request_after_update
        }
    }
}


use super::super::DBLoad;
use super::super::tiberius_interface::FromOwnedSql;

dbload!(Sheet, "SHEET", COL_PK, COL_DESCRIPTION, COL_TABLE_NAME, COL_LAST_EDITED_BY_FK, COL_ACTIVE, COL_DAYS_TO_REFRESH?, COL_MODEL?, COL_REQUEST_AFTER_UPDATE?);
