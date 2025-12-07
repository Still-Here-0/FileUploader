use chrono::NaiveDateTime;
use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct Upload {
    sheet_fk: i32,
    file_uploaded: Vec<u8>,
    uploaded_at: NaiveDateTime,
    uploaded_by_fk: i32,
    sheet_used: Option<String>
}

impl Upload {
    pub const COL_SHEET_FK: &'static str = "Sheet_fk";
    pub const COL_FILE_UPLOADED: &'static str = "FileUploaded";
    pub const COL_UPLOADED_AT: &'static str = "UploadedAt";
    pub const COL_UPLOADED_BY_FK: &'static str = "UploadedBy_fk";
    pub const COL_SHEET_USED: &'static str = "SheetUsed";
    
    pub fn db_new(
        sheet_fk: i32,
        file_uploaded: &[u8],
        uploaded_at: NaiveDateTime,
        uploaded_by_fk: i32,
        sheet_used: Option<&str>
    ) -> Self {
        let file_uploaded = file_uploaded.to_vec();
        let sheet_used = sheet_used.map(str::to_string);

        Self {
            sheet_fk,
            file_uploaded,
            uploaded_at,
            uploaded_by_fk,
            sheet_used
        }
    }
}

use super::super::DBLoad;

dbload!(Upload, "UPLOAD", COL_SHEET_FK, COL_FILE_UPLOADED, COL_UPLOADED_AT, COL_UPLOADED_BY_FK, COL_SHEET_USED?);
