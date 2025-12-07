use chrono::NaiveDateTime;
use macros::dbload;
use serde::{Deserialize, Serialize, de::value};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistGroup {
    group_fk: i32,
    name: Option<String>,
    active: Option<bool>,
    add_worker: Option<bool>,
    edit_worker: Option<bool>,
    add_profile: Option<bool>,
    remove_profile: Option<bool>,
    add_group: Option<bool>,
    remove_group: Option<bool>,
    edit_group: Option<bool>,
    edit_profile_groups: Option<bool>,
    edited_by_fk: i32,
    edited_at: NaiveDateTime,
    edit_action: String,
    impersonate_users: Option<bool>
}

impl HistGroup {
    pub const COL_GROUP_FK: &'static str = "Group_fk";
    pub const COL_NAME: &'static str = "Name";
    pub const COL_ACTIVE: &'static str = "Active";
    pub const COL_ADD_WORKER: &'static str = "AddWorker";
    pub const COL_EDIT_WORKER: &'static str = "EditWorker";
    pub const COL_ADD_PROFILE: &'static str = "AddProfile";
    pub const COL_REMOVE_PROFILE: &'static str = "RemoveProfile";
    pub const COL_ADD_GROUP: &'static str = "AddGroup";
    pub const COL_REMOVE_GROUP: &'static str = "RemoveGroup";
    pub const COL_EDIT_GROUP: &'static str = "EditGroup";
    pub const COL_EDIT_PROFILE_GROUPS: &'static str = "EditProfileGroups";
    pub const COL_IMPERSONATE_USERS: &'static str = "ImpersonateUsers";
    
    pub const COL_EDITED_BY_FK: &'static str = "EditedBy_fk";
    pub const COL_EDITED_AT: &'static str = "EditedAt";
    pub const COL_EDIT_ACTION: &'static str = "EditAction";
    
    pub fn db_new(
        group_fk: i32,
        name: Option<&str>,
        active: Option<bool>,
        add_worker: Option<bool>,
        edit_worker: Option<bool>,
        add_profile: Option<bool>,
        remove_profile: Option<bool>,
        add_group: Option<bool>,
        remove_group: Option<bool>,
        edit_group: Option<bool>,
        edit_profile_groups: Option<bool>,
        edited_by_fk: i32,
        edited_at: NaiveDateTime,
        edit_action: &str,
        impersonate_users: Option<bool>
    ) -> Self {
        let name = name.map(str::to_string);
        let edit_action = edit_action.to_string();
        
        Self {
            group_fk,
            name,
            active,
            add_worker,
            edit_worker,
            add_profile,
            remove_profile,
            add_group,
            remove_group,
            edit_group,
            edit_profile_groups,
            edited_by_fk,
            edited_at,
            edit_action,
            impersonate_users
        }
    }
}


use super::super::DBLoad;

dbload!(HistGroup, "HIST_GROUP", COL_GROUP_FK, COL_NAME?, COL_ACTIVE?, COL_ADD_WORKER?, COL_EDIT_WORKER?, COL_ADD_PROFILE?, COL_REMOVE_PROFILE?, COL_ADD_GROUP?, COL_REMOVE_GROUP?, COL_EDIT_GROUP?, COL_EDIT_PROFILE_GROUPS?, COL_EDITED_BY_FK, COL_EDITED_AT, COL_EDIT_ACTION, COL_IMPERSONATE_USERS?);
