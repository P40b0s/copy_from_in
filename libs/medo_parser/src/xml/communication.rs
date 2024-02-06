use std::{fmt::{Debug}};

use serde::{Serialize, Deserialize};

use crate::{Header, RootContainer, Files, Document, DeliveryIndex, Acknowledgment};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
///основное пространство в руте, контейнер который содердится в 2.7.1
/// получается лежит не тут а как бы другой файл
pub struct Communication 
{
    #[serde(rename="@version")]
    pub version: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub header: Option<Header>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub container: Option<RootContainer>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub acknowledgment: Option<Acknowledgment>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub delivery_index: Option<DeliveryIndex>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub document : Option<Document>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub files : Option<Files>,
}

impl Default for Communication
{
    fn default() -> Self 
    {
        Communication 
        { 
            version: "0.0.0".to_owned(),
            header: None,
            container: None,
            delivery_index: None,
            document: None,
            files: None,
            acknowledgment: None
        }
    }
}