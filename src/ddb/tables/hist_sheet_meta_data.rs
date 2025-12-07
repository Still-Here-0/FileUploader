use chrono::NaiveDateTime;
use macros::dbload;
use serde::{Deserialize, Serialize, de::value};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistSheetMetaData {
    sheet_meta_data_fk: i32,
    sheet_fk: Option<i32>,
    column_name: Option<String>,
    column_type_fk: Option<i32>,
    optinal: Option<bool>,
    regex_constrait: Option<String>,
    description: Option<String>,
    edited_by_fk: i32,
    edited_at: NaiveDateTime,
    edit_action: String,
}

impl HistSheetMetaData {
    pub const COL_SHEET_META_DATA_FK: &'static str = "SheetMetaData_fk";
    pub const COL_SHEET_FK: &'static str = "Sheet_fk";
    pub const COL_COLUMN_NAME: &'static str = "ColumnName";
    pub const COL_COLUMN_TYPE_FK: &'static str = "ColumnType_fk";
    pub const COL_OPTIONAL: &'static str = "Optional";
    pub const COL_REGEX_CONSTRAINT: &'static str = "RegexConstraint";
    pub const COL_DESCRIPTION: &'static str = "Description";

    pub const COL_EDITED_BY_FK: &'static str = "EditedBy_fk";
    pub const COL_EDITED_AT: &'static str = "EditedAt";
    pub const COL_EDIT_ACTION: &'static str = "EditAction";
    
    pub fn db_new(
        sheet_meta_data_fk: i32,
        sheet_fk: Option<i32>,
        column_name: Option<&str>,
        column_type_fk: Option<i32>,
        optinal: Option<bool>,
        regex_constrait: Option<&str>,
        description: Option<&str>,
        edited_by_fk: i32,
        edited_at: NaiveDateTime,
        edit_action: &str,
    ) -> Self {
        let column_name = column_name.map(str::to_string);
        let regex_constrait = regex_constrait.map(str::to_string);
        let description = description.map(str::to_string);
        let edit_action = edit_action.to_string();
        
        Self {
            sheet_meta_data_fk,
            sheet_fk,
            column_name,
            column_type_fk,
            optinal,
            regex_constrait,
            description,
            edited_by_fk,
            edited_at,
            edit_action,
        }
    }
}


use super::super::DBLoad;

dbload!(HistSheetMetaData, "HIST_SHEET_META_DATA", COL_SHEET_META_DATA_FK, COL_SHEET_FK?, COL_COLUMN_NAME?, COL_COLUMN_TYPE_FK?, COL_OPTIONAL?, COL_REGEX_CONSTRAINT?, COL_DESCRIPTION?, COL_EDITED_BY_FK, COL_EDITED_AT, COL_EDIT_ACTION);
