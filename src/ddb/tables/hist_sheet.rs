use chrono::NaiveDateTime;
use macros::dbload;
use serde::{Deserialize, Serialize, de::value};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistSheet {
    sheet_fk: i32,
    description: Option<String>,
    table_name: Option<String>,
    last_edited_by_fk: Option<i32>,
    active: Option<bool>,
    days_to_refresh: Option<i32>,
    model: Option<Vec<u8>>,
    request_after_update: Option<String>,
    edited_by_fk: i32,
    edited_at: NaiveDateTime,
    edit_action: String,
}

impl HistSheet {
    pub const COL_SHEET_FK: &'static str = "Sheet_fk";
    pub const COL_DESCRIPTION: &'static str = "Description";
    pub const COL_TABLE_NAME: &'static str = "TableName";
    pub const COL_LAST_EDITED_BY_FK: &'static str = "LastEditedBy_fk";
    pub const COL_ACTIVE: &'static str = "Active";
    pub const COL_DAYS_TO_REFRESH: &'static str = "DaysToRefresh";
    pub const COL_MODEL: &'static str = "Model";
    pub const COL_REQUEST_AFTER_UPDATE: &'static str = "RequestAfterUpdate";
    
    pub const COL_EDITED_BY_FK: &'static str = "EditedBy_fk";
    pub const COL_EDITED_AT: &'static str = "EditedAt";
    pub const COL_EDIT_ACTION: &'static str = "EditAction";
    
    pub fn db_new(
        sheet_fk: i32,
        description: Option<String>,
        table_name: Option<String>,
        last_edited_by_fk: Option<i32>,
        active: Option<bool>,
        days_to_refresh: Option<i32>,
        model: Option<Vec<u8>>,
        request_after_update: Option<String>,
        edited_by_fk: i32,
        edited_at: NaiveDateTime,
        edit_action: String,
    ) -> Self {
        Self {
            sheet_fk,
            description,
            table_name,
            last_edited_by_fk,
            active,
            days_to_refresh,
            model,
            request_after_update,
            edited_by_fk,
            edited_at,
            edit_action,
        }
    }
}


use super::super::DBLoad;
use super::super::tiberius_interface::FromOwnedSql;

dbload!(HistSheet, "HIST_SHEET", COL_SHEET_FK, COL_DESCRIPTION?, COL_TABLE_NAME?, COL_LAST_EDITED_BY_FK?, COL_ACTIVE?, COL_DAYS_TO_REFRESH?, COL_MODEL?, COL_REQUEST_AFTER_UPDATE?, COL_EDITED_BY_FK, COL_EDITED_AT, COL_EDIT_ACTION);
