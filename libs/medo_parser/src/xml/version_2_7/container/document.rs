use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document
{
    #[serde(rename="@localName")]
    pub local_name: String,
    pub pages_quantity: u32,
    #[serde(skip_serializing_if="Option::is_none")]
    pub enclosure_pages_quantity: Option<u32>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>
}

