use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attachment
{
    #[serde(rename="@localName")]
    local_name: String,
    order: u32,
    #[serde(skip_serializing_if="Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub signature: Option<Vec<Signature>>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Signature
{
    #[serde(rename="@localName")]
    pub local_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attachments
{
    #[serde(rename="attachment")]
    pub attachments : Vec<Attachment>
}

