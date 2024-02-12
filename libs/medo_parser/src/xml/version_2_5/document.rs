use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use crate::{Uid, guid_deserializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document
{
    #[serde(deserialize_with="guid_deserializer")]
    #[serde(rename="@uid")]
    pub uid: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub num: Option<Number>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub signatories: Option<Signatories>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub addressees: Option<Addressees>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pages: Option<u32>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotation: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub correspondents: Option<Correspondents>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub executor: Option<Executor>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Number
{
    #[serde(skip_serializing_if="Option::is_none")]
    ///<xdms:number>78</xdms:number>
    pub number: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///<xdms:date>2023-01-25</xdms:date>
    pub date: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Signatory
{
    #[serde(skip_serializing_if="Option::is_none")]
    ///ФИО - Иванов И.И.
    pub person: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Секретариат Иванова И.И.
    pub department: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Председатель ООО "рога и копыта"
    pub post: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///2023-01-24
    pub signed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Signatories
{
    #[serde(rename="signatory")]
    pub signatories: Vec<Signatory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
///Тут адрес отправителя и реквизиты исполнителя
pub struct Addressee
{
    #[serde(skip_serializing_if="Option::is_none")]
    ///г. Москва
    region: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///ООО "рога и копыта"
    organization: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Зиц
    person: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Департамент по заготовке копыт
    department: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Сидящий за других
    post: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Addressees
{
    #[serde(rename="addressee")]
    addressees: Vec<Addressee>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
///Тут адрес отправителя и реквизиты исполнителя
pub struct Correspondent
{
    #[serde(skip_serializing_if="Option::is_none")]
    ///г. Москва
    region: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///ООО "рога и копыта"
    organization: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Зиц
    person: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Департамент по заготовке копыт
    department: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Сидящий за других
    post: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    num: Option<Number>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Correspondents
{
    #[serde(rename="correspondent")]
    correspondents: Vec<Correspondent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
///Тут адрес отправителя и реквизиты исполнителя
pub struct Executor
{
    #[serde(skip_serializing_if="Option::is_none")]
    ///г. Москва
    pub region: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///ООО "рога и копыта"
    pub organization: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Зиц
    pub person: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Сидящий за других
    pub post: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Департамент по заготовке копыт
    pub contact_info: Option<String>,
}

impl Uid for Document
{
    fn get_uid(&self) -> Cow<str>
    {
        Cow::from(&self.uid)
    }
}