use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagerPermission {
    group_fk: i32,
    add_worker: bool,
    edit_worker: bool,
    add_profile: bool,
    remove_profile: bool,
    add_group: bool,
    remove_group: bool,
    edit_group: bool,
    edit_profile_groups: bool,
    impersonate_users: bool,
}

impl ManagerPermission {
    pub const COL_GROUP_FK: &'static str = "Group_fk";
    pub const COL_ADD_WORKER: &'static str = "AddWorker";
    pub const COL_EDIT_WORKER: &'static str = "EditWorker";
    pub const COL_ADD_PROFILE: &'static str = "AddProfile";
    pub const COL_REMOVE_PROFILE: &'static str = "RemoveProfile";
    pub const COL_ADD_GROUP: &'static str = "AddGroup";
    pub const COL_REMOVE_GROUP: &'static str = "RemoveGroup";
    pub const COL_EDIT_GROUP: &'static str = "EditGroup";
    pub const COL_EDIT_PROFILE_GROUPS: &'static str = "EditProfileGroups";
    pub const COL_IMPERSONATE_USERS: &'static str = "ImpersonateUsers";
    
    pub fn db_new(
        group_fk: i32,
        add_worker: bool,
        edit_worker: bool,
        add_profile: bool,
        remove_profile: bool,
        add_group: bool,
        remove_group: bool,
        edit_group: bool,
        edit_profile_groups: bool,
        impersonate_users: bool,
    ) -> Self {
        Self {
            group_fk,
            add_worker,
            edit_worker,
            add_profile,
            remove_profile,
            add_group,
            remove_group,
            edit_group,
            edit_profile_groups,
            impersonate_users,
        }
    }
}


use super::super::DBLoad;

dbload!(ManagerPermission, "MANAGER_PERMISSION", COL_GROUP_FK, COL_ADD_WORKER, COL_EDIT_WORKER, COL_ADD_PROFILE, COL_REMOVE_PROFILE, COL_ADD_GROUP, COL_REMOVE_GROUP, COL_EDIT_GROUP, COL_EDIT_PROFILE_GROUPS, COL_IMPERSONATE_USERS );
