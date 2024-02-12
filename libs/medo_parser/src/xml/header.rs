use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use crate::{Uid, guid_deserializer, Source};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename="header")]
#[serde(rename_all = "camelCase")]
pub struct Header
{
    ///126b3150-c538-40de-88c7-e8674b5d7843
    #[serde(deserialize_with="guid_deserializer")]
    #[serde(rename="@uid")]
    uid: String,
    #[serde(rename="@type")]
    ///type="Транспортный контейнер"
    p_type: String,
    ///created="2022-05-19T13:18:26+07:00"
    #[serde(rename="@created")]
    #[serde(skip_serializing_if="Option::is_none")]
    created: Option<String>,
    source: Source,
    #[serde(skip_serializing_if="Option::is_none")]
    comment: Option<String>,

}

impl Header
{
    pub fn get_source(&self) -> &Source
    {
        &self.source
    }
    pub fn get_type(&self) -> Cow<str>
    {
        Cow::from(&self.p_type)
    }
}

impl Uid for Header
{
    fn get_uid(&self) -> Cow<str>
    {
        Cow::from(&self.uid)
    }
}