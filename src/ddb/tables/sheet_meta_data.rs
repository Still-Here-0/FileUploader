use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct SheetMetaData {
    pk: i32,
    sheet_fk: i32,
    column_name: String,
    column_type_fk: i32,
    optinal: bool,
    regex_constrait: Option<String>,
    last_editeded_by_fk: i32,
    description: String
}

impl SheetMetaData {
    pub const COL_PK: &'static str = "pk";
    pub const COL_SHEET_FK: &'static str = "Sheet_fk";
    pub const COL_COLUMN_NAME: &'static str = "ColumnName";
    pub const COL_COLUMN_TYPE_FK: &'static str = "ColumnType_fk";
    pub const COL_OPTIONAL: &'static str = "Optional";
    pub const COL_REGEX_CONSTRAINT: &'static str = "RegexConstraint";
    pub const COL_LAST_EDITED_BY_FK: &'static str = "LastEditedBy_fk";
    pub const COL_DESCRIPTION: &'static str = "Description";
    
    pub fn db_new(
        pk: i32,
        sheet_fk: i32,
        column_name: String,
        column_type_fk: i32,
        optinal: bool,
        regex_constrait: Option<String>,
        last_editeded_by_fk: i32,
        description: String
    ) -> Self {
        Self {
            pk,
            sheet_fk,
            column_name,
            column_type_fk,
            optinal,
            regex_constrait,
            last_editeded_by_fk,
            description
        }
    }
}


use super::super::DBLoad;
use super::super::tiberius_interface::FromOwnedSql;

dbload!(SheetMetaData, "SHEET_META_DATA", COL_PK, COL_SHEET_FK, COL_COLUMN_NAME, COL_COLUMN_TYPE_FK, COL_OPTIONAL, COL_REGEX_CONSTRAINT?, COL_LAST_EDITED_BY_FK, COL_DESCRIPTION);
