use chrono::NaiveDateTime;
use macros::dbload;
use serde::{Deserialize, Serialize, de::value};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistUploaderPermission {
    group_fk: i32,
    sheet_fk: i32,
    can_view_hist: Option<String>,
    can_upload: Option<i32>,
    edited_by_fk: i32,
    edited_at: NaiveDateTime,
    edit_action: String,
}

impl HistUploaderPermission {
    pub const COL_GROUP_FK: &'static str = "Group_fk";
    pub const COL_SHEET_FK: &'static str = "Sheet_fk";
    pub const COL_CAN_VIEW_HIST: &'static str = "CanViewHist";
    pub const COL_CAN_UPLOAD: &'static str = "CanUpload";

    pub const COL_EDITED_BY_FK: &'static str = "EditedBy_fk";
    pub const COL_EDITED_AT: &'static str = "EditedAt";
    pub const COL_EDIT_ACTION: &'static str = "EditAction";
    
    pub fn db_new(
        group_fk: i32,
        sheet_fk: i32,
        can_view_hist: Option<&str>,
        can_upload: Option<i32>,
        edited_by_fk: i32,
        edited_at: NaiveDateTime,
        edit_action: &str,
    ) -> Self {
        let can_view_hist = can_view_hist.map(str::to_string);
        let edit_action = edit_action.to_string();
        
        Self {
            group_fk,
            sheet_fk,
            can_view_hist,
            can_upload,
            edited_by_fk,
            edited_at,
            edit_action,
        }
    }
}


use super::super::DBLoad;

dbload!(HistUploaderPermission, "HIST_UPLOADER_PERMISSION", COL_GROUP_FK, COL_SHEET_FK, COL_CAN_VIEW_HIST?, COL_CAN_UPLOAD?, COL_EDITED_BY_FK, COL_EDITED_AT, COL_EDIT_ACTION);
