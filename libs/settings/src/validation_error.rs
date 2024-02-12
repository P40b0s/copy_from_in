use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
//#[serde(rename_all = "camelCase")]
pub struct ValidationError
{
    pub field_name: Option<String>,
    pub error: String
}
impl ValidationError
{
    pub fn new(field_name: Option<String>, error: String)-> Self
    {
        Self 
        { 
            field_name, 
            error 
        }
    }
    pub fn new_from_str(field_name: Option<String>, error: &str)-> Self
    {
        Self 
        { 
            field_name, 
            error: error.to_owned()
        }
    }
}
impl Display for ValidationError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        if let Some(field) = self.field_name.as_ref()
        {   let msg = ["Ошибка настроек в поле ", field, " -> ", &self.error].concat();
            f.write_str(&msg)
        }   
        else
        {
            f.write_str(&self.error)
        }
    }
}
