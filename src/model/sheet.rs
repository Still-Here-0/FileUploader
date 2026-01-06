use serde::{Deserialize, Serialize};





#[derive(Debug, Serialize, Deserialize)]
pub struct NewSheetRequest {
    pub description: String,
    pub table_name: String,
    pub days_to_refresh: Option<i32>,
    pub request_after_update: Option<String>
}