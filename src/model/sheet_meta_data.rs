use serde::{Deserialize, Serialize};





#[derive(Debug, Serialize, Deserialize)]
pub struct NewSheetMetaDataRequest {
    pub name: String,
    pub column_type_fk: i32,
    pub optional: bool,
    pub regex_constraint: Option<String>,
    pub description: String
}