use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phones 
{
    pub phone_type: String,
    pub phone_number: String,
    pub is_main: bool
}

