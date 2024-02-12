use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
///Пример: Росстат
pub struct Organization
{
    pub title: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>
}