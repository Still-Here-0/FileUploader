use chrono::NaiveDateTime;
use macros::dbload;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use serde_json::map::ValuesMut;
use std::{pin::Pin, result};
use tiberius::QueryStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
    pk: i32,
    name: String,
    linde_id: String,
    email: String,
}

impl Worker {
    pub const COL_PK: &'static str = "pk";
    pub const COL_NAME: &'static str = "Name";
    pub const COL_LINDE_ID: &'static str = "LindeId";
    pub const COL_EMAIL: &'static str = "Email";
    
    pub fn db_new(
        pk: i32,
        name: &str,
        linde_id: &str,
        email: &str,
    ) -> Self {
        let name = name.to_string();
        let linde_id  = linde_id.to_string();
        let email = email.to_string();

        Self {
            pk,
            name,
            linde_id,
            email,
        }
    }
}

use super::super::DBLoad;

dbload!(Worker, "WORKER", COL_PK, COL_NAME, COL_LINDE_ID, COL_EMAIL);
