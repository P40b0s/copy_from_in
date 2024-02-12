use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File
{
    #[serde(rename="@localName")]
    pub local_name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="@localId")]
    pub local_id: Option<u32>,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="@type")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pages: Option<u32>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Files
{
    #[serde(rename="file")]
    pub files: Vec<File>
}