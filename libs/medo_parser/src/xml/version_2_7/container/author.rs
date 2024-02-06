use serde::{Serialize, Deserialize};
use super::Organization;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
///Автор
pub struct Author
{
    pub organization: Organization,
    pub registration: Registration,
    pub sign: Vec<Sign>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub executor: Option<Executor>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub department: Option<String>

}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Registration
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
    pub registration_stamp: RegistrationStamp
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationStamp
{
    #[serde(rename="@localName")]
    pub local_name: String,
    pub position: Position
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position
{
    pub page: u32,
    pub top_left: TopLeft,
    pub dimension: Dimension
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TopLeft
{
    pub x: u32,
    pub y: u32
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dimension
{
    pub w: u32,
    pub h: u32
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sign
{
    pub person: Person,
    pub document_signature: DocumentSignature
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Person
{
    pub post: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSignature
{
    #[serde(rename="@localName")]
    pub local_name: String,
    pub signature_stamp: SignatureStamp
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignatureStamp
{
    pub position: Position
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Executor
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub post: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub email: Option<String>
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Authors
{
    #[serde(rename="author")]
    pub authors: Vec<Author>
}