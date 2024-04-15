use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewDocument
{
    pub organization: Option<String>,
    pub organization_uid: Option<String>,
    pub doc_type: Option<String>,
    pub doc_uid: Option<String>,
    pub number: Option<String>,
    pub sign_date: Option<String>,
    pub source_medo_addressee: Option<String>,
}
