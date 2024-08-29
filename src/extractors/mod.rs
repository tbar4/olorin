pub mod file_reader;
pub mod json_api;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub name: String,
    pub r#type: String,
    pub url: String,
    pub results_field_name: String,
    pub schema: Vec<Value> 
}