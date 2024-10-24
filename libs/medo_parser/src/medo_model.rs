use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Requisites
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub document_guid: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub act_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub document_number: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sign_date: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pages: Option<u32>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotation: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub mj: Option<MinistryOfJustice>
}
impl Default for Requisites
{
    fn default() -> Self 
    {
        Requisites
        {
            document_guid: None,
            act_type: None,
            document_number: None,
            sign_date: None,
            pages: None,
            annotation: None,
            mj: None,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicationInfo
{
    pub number: String,
    pub date: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PacketInfo
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub header_guid : Option<String>,
    pub packet_directory: String,
    pub packet_type: String,
    ///Время создания локальной директории
    ///(фактически когда пакет пришел к нам)
    ///зависит от времени на сервере, тому что берет локальное время создания
    pub delivery_time : String,
    pub wrong_encoding: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<String>,
    pub files: Vec<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub requisites: Option<Requisites>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sender_info: Option<SenderInfo>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_pdf: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pdf_hash: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub acknowledgment: Option<Ack>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub trace_message: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub publication_info: Option<PublicationInfo>,
    //время обновления пакета
    pub update_key: String,
    pub visible: bool,
}

impl PacketInfo
{
    pub fn get_packet_dir(&self) -> PathBuf
    {
       Path::new(&self.packet_directory).to_owned()
    }
}

impl Default for PacketInfo
{
    fn default() -> Self
    {
        PacketInfo 
        { 
            header_guid: None,
            packet_directory: String::from("Ошибка преобразования!"),
            error: None,
            files: vec![],
            requisites: None,
            sender_info: None,
            default_pdf: None,
            pdf_hash: None,
            acknowledgment: None,
            wrong_encoding: false,
            packet_type: "неизвестно".to_owned(),
            delivery_time: "01-01-2000T00:00:00".to_owned(),
            trace_message: None,
            publication_info: None,
            update_key: "01-01-2000T00:00:00".to_owned(),
            visible: true
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SenderInfo
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub person: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub post: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub addressee: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub medo_addessee: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_guid: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub executor: Option<Executor>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Executor
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub person: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub post: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub contact_info: Option<String>,
}

impl Default for SenderInfo
{
    fn default() -> Self 
    {
        SenderInfo 
        {
            organization: None,
            person: None,
            department: None,
            post: None,
            addressee: None,
            medo_addessee: None,
            source_guid: None,
            executor: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinistryOfJustice
{
    pub number: String,
    pub date: String
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ack
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<String>,
    pub accepted: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub time: Option<String>
}