use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use super::{Requisites, Document, Attachments, Addresseess, Authors};
use crate::{guid_deserializer, Uid};

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// #[serde(rename="containerRoot")]
// #[serde(rename_all = "camelCase")]
// ///Контейнер который содердержиться в 2.7.1
// pub struct ContainerRoot
// {
//     container: Container
// }

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
///Контейнер который содердержиться в 2.7.1
pub struct Container
{
    #[serde(deserialize_with="guid_deserializer")]
    #[serde(rename="@uid")]
    pub uid: String,
    #[serde(rename="@version")]
    pub version: String,
    pub requisites: Requisites,
    pub authors: Authors,
    pub addressees: Addresseess,
    pub document: Document,
    #[serde(skip_serializing_if="Option::is_none")]
    pub attachments: Option<Attachments>
   
}

impl Uid for Container
{
    fn get_uid(&self) -> Cow<str>
    {
        Cow::from(&self.uid)
    }
}