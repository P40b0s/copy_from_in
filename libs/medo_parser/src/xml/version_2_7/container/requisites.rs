use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Requisites
{
    pub document_kind: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub document_place: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub classification: Option<String>,
    pub annotation: String,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdStringText
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[serde(rename="$value")]
    pub content: String,
}

