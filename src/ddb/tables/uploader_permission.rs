use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploaderPermission {
    group_fk: i32,
    sheet_fk: i32,
    can_view_hist: bool,
    can_upload: bool,
    last_edited_by_fk: i32,
}

impl UploaderPermission {
    pub const COL_GROUP_FK: &'static str = "Group_fk";
    pub const COL_SHEET_FK: &'static str = "Sheet_fk";
    pub const COL_CAN_VIEW_HIST: &'static str = "CanViewHist";
    pub const COL_CAN_UPLOAD: &'static str = "CanUpload";
    pub const COL_LAST_EDITED_BY_FK: &'static str = "LastEditedBy_fk";
    
    pub fn db_new(
        group_fk: i32,
        sheet_fk: i32,
        can_view_hist: bool,
        can_upload: bool,
        last_edited_by_fk: i32,
    ) -> Self {
        Self {
            group_fk,
            sheet_fk,
            can_view_hist,
            can_upload,
            last_edited_by_fk,
        }
    }
}

use super::super::DBLoad;
use super::super::tiberius_interface::FromOwenedSql;

dbload!(UploaderPermission, "UPLOADER_PERMISSION", COL_GROUP_FK, COL_SHEET_FK, COL_CAN_VIEW_HIST, COL_CAN_UPLOAD, COL_LAST_EDITED_BY_FK);
