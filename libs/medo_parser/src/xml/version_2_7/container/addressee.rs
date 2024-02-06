use serde::{Serialize, Deserialize};
use super::Organization;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Addressee
{
    organization: Organization
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Addresseess
{
    #[serde(rename="addressee")]
    addresseess: Vec<Addressee>
}

