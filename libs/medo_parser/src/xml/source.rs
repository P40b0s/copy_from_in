use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use crate::{Uid, guid_deserializer};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Source
{
    ///Аттирибут
    #[serde(deserialize_with="guid_deserializer")]
    #[serde(rename="@uid")]
    uid: String,
    organization: String
}

impl Source
{
    pub fn get_organization(&self) -> Cow<str>
    {
        Cow::from(&self.organization)
    }
}

impl Uid for Source
{
    fn get_uid(&self) -> Cow<str>
    {
        Cow::from(&self.uid)
    }
}