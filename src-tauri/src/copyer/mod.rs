mod directories_spy;
mod service;
pub use  {directories_spy::DirectoriesSpy, service::{PacketsCleaner, ExcludesCreator}};
use medo_parser::Packet;
use serde::{Deserialize, Serialize};
use settings::{DateTimeFormat, Task, ValidationError};
mod io;
mod serialize;

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
// impl NewDocument
// {
//     pub fn new(packet_name: &str) -> Self
//     {
//         Self
//         {
//             organization: None,
//             organization_uid: None,
//             doc_type: None,
//             doc_uid: None,
//             number: None,
//             sign_date: None,
//             source_medo_addressee: None,
//             //name: packet_name.to_owned(),
//             //parse_time: settings::Date::now().as_serialized()
//         }
//     }
// }

impl From<&Packet> for NewDocument
{
    fn from(value: &Packet) -> Self 
    {
        let organization = value.get_organization().map_or( None, |o| Some(o.into_owned()));
        let organization_uid = value.get_source_uid().map_or( None, |o| Some(o.into_owned()));
        let date_number = value.get_document_date_number();
        let doc_uid = value.get_document_uid();
        let source_medo_addressee = value.get_source_addressee();
        let date = date_number.as_ref().and_then(|d| d.date.clone());
        let number = date_number.as_ref().and_then(|d| d.number.clone());
        Self
        {
            organization,
            organization_uid,
            doc_type: value.get_document_type(),
            doc_uid,
            number,
            sign_date: date,
            source_medo_addressee,
            //name: value.get_packet_name().to_owned(),
            //parse_time: settings::Date::now().as_serialized()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewPacketInfo
{
    document: Option<NewDocument>,
    error: Option<String>,
    task: Task,
    pub name: String,
    pub parse_time: String,
    pub report_sended: bool
}
impl NewPacketInfo
{
    pub fn get_packet_name(&self) -> &str
    {
        &self.name
    }
    pub fn from_err(err: &str, packet_name: &str, task: &Task) -> Self
    {
        Self
        {
            name: packet_name.to_owned(),
            parse_time: settings::Date::now().as_serialized(),
            report_sended: false,
            document: None,
            error: Some(err.to_owned()),
            task: task.clone()
        }
    }
    pub fn from_packet(packet: &Packet, task: &Task) -> Self
    {
        Self
        {
            name: packet.get_packet_name().to_owned(),
            parse_time: settings::Date::now().as_serialized(),
            document: Some(packet.into()),
            error: None,
            report_sended: false,
            task: task.clone()
        }
    }
    pub fn not_packet(packet_dir: &str, task: &Task) -> Self
    {
        Self
        {
            name: packet_dir.to_owned(),
            parse_time: settings::Date::now().as_serialized(),
            document: None,
            error: None,
            report_sended: false,
            task: task.clone()
        }
    }
}

// impl From<&Vec<ValidationError>> for NewPacketInfo
// {
//     fn from(value: &Vec<ValidationError>) -> Self 
//     {
//         let mut errors = String::new();
//         let error = value.iter().fold(&mut errors, |acc, val|
//         {
//             let str = [val.to_string(), "\\n".to_owned()].concat();
//             acc.push_str(&str);
//             acc
//         });
//         Self
//         {
//             document: None,
//             error: Some(error.clone()),
//             task: None
//         }
//     }
// }

// impl From<&Packet> for NewPacketInfo
// {
//     fn from(value: &Packet) -> Self 
//     {
//         Self
//         {
//             name: value.get_packet_name().to_owned(),
//             parse_time: settings::Date::now().as_serialized(),
//             document: Some(value.into()),
//             error: None,
//             task: None
//         }
//     }
// }
// impl From<NewDocument> for NewPacketInfo
// {
//     fn from(value: NewDocument) -> Self 
//     {
//         Self
//         {
//             name: value.get_packet_name().to_owned(),
//             parse_time: settings::Date::now().as_serialized(),
//             document: Some(value),
//             error: None,
//             task: None
//         }
//     }
// }

// impl From<&NewDocument> for NewPacketInfo
// {
//     fn from(value: &NewDocument) -> Self 
//     {
//         Self
//         {
//             document: Some(value.to_owned()),
//             error: None,
//             task: None
//         }
//     }
// }

// impl From<String> for NewPacketInfo
// {
//     fn from(value: String) -> Self 
//     {
//         Self
//         {
//             document: None,
//             error: Some(value),
//             task: None
//         }
//     }
// }