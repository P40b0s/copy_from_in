use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use utilites::Date;
#[cfg(feature = "all")]
use crate::packet::Packet;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
#[cfg(any(feature = "model", feature = "all"))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PacketInfo
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub header_guid : Option<String>,
    pub packet_directory: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub packet_type: Option<String>,
    ///Время создания локальной директории
    ///(фактически когда пакет пришел к нам)
    ///зависит от времени на сервере, тому что берет локальное время создания
    pub delivery_time : String,
    pub wrong_encoding: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub error: Option<(i8, String)>,
    pub files: Vec<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub requisites: Option<Requisites>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sender_info: Option<SenderInfo>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_pdf: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub pdf_hash: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub acknowledgment: Option<Ack>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub trace_message: Option<String>,
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
    #[cfg(feature = "all")]
    pub fn parse<P: AsRef<Path>>(path: P) -> Self
    {
        Packet::parse(path).into()
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
            sender_id: None,
            default_pdf: None,
            pdf_hash: None,
            acknowledgment: None,
            wrong_encoding: false,
            packet_type: None,
            delivery_time: Date::now().format(utilites::DateFormat::Serialize),
            trace_message: None,
            update_key: Date::now().format(utilites::DateFormat::Serialize),
            visible: true
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    pub medo_addressee: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_guid: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub executor: Option<Executor>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon: Option<String>
}
impl SenderInfo
{
    pub fn is_null(&self) -> bool
    {
        self.organization.is_none()
        &&  self.person.is_none()
        &&  self.department.is_none()
        &&  self.post.is_none()
        &&  self.addressee.is_none()
        &&  self.medo_addressee.is_none()
        &&  self.source_guid.is_none()
        &&  self.executor.is_none()  
        &&  self.icon.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
            medo_addressee: None,
            source_guid: None,
            executor: None,
            icon: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MinistryOfJustice
{
    pub number: String,
    pub date: String
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ack
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub comment: Option<String>,
    pub accepted: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub time: Option<String>
}