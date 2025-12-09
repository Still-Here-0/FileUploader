use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileGroups {
    profile_fk: i32,
    group_fk: i32,
}

impl ProfileGroups {
    pub const COL_PROFILE_FK: &'static str = "Profile_fk";
    pub const COL_GROUP_FK: &'static str = "Group_fk";
    
    pub fn db_new(
        profile_fk: i32,
        group_fk: i32,
    ) -> Self {
        Self {
            profile_fk,
            group_fk,
        }
    }
}


use super::super::DBLoad;
use super::super::tiberius_interface::FromOwnedSql;

dbload!(ProfileGroups, "PROFILE_GROUPS", COL_PROFILE_FK, COL_GROUP_FK);
