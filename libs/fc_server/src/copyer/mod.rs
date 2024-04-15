mod directories_spy;
mod service;
pub use  {directories_spy::DirectoriesSpy, directories_spy::get_full_log, service::{PacketsCleaner, ExcludesCreator}};
use medo_parser::Packet;
use serde::{Deserialize, Serialize};
use settings::{DateTimeFormat, Task};
use transport::{NewDocument, NewPacketInfo};
mod io;
mod serialize;

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct NewDocument
// {
//     pub organization: Option<String>,
//     pub organization_uid: Option<String>,
//     pub doc_type: Option<String>,
//     pub doc_uid: Option<String>,
//     pub number: Option<String>,
//     pub sign_date: Option<String>,
//     pub source_medo_addressee: Option<String>,
// }

// impl From<&Packet> for NewDocument
// {
//     fn from(value: &Packet) -> Self 
//     {
//         let organization = value.get_organization().map_or( None, |o| Some(o.into_owned()));
//         let organization_uid = value.get_source_uid().map_or( None, |o| Some(o.into_owned()));
//         let date_number = value.get_document_date_number();
//         let doc_uid = value.get_document_uid();
//         let source_medo_addressee = value.get_source_addressee();
//         let date = date_number.as_ref().and_then(|d| d.date.clone());
//         let number = date_number.as_ref().and_then(|d| d.number.clone());
//         Self
//         {
//             organization,
//             organization_uid,
//             doc_type: value.get_document_type(),
//             doc_uid,
//             number,
//             sign_date: date,
//             source_medo_addressee,
//         }
//     }
// }

pub trait AsNewdoc<I, O>
{
    fn as_doc(self) -> O; 
}

impl AsNewdoc<Packet, NewDocument> for &Packet
{
    fn as_doc(self) -> NewDocument 
    {
        let organization = self.get_organization().map_or( None, |o| Some(o.into_owned()));
        let organization_uid = self.get_source_uid().map_or( None, |o| Some(o.into_owned()));
        let date_number = self.get_document_date_number();
        let doc_uid = self.get_document_uid();
        let source_medo_addressee = self.get_source_addressee();
        let date = date_number.as_ref().and_then(|d| d.date.clone());
        let number = date_number.as_ref().and_then(|d| d.number.clone());
        NewDocument 
        {
            organization,
            organization_uid,
            doc_type: self.get_document_type(),
            doc_uid,
            number,
            sign_date: date,
            source_medo_addressee,
        }
    }
}

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct NewPacketInfo
// {
//     document: Option<NewDocument>,
//     error: Option<String>,
//     task: Task,
//     pub name: String,
//     pub parse_time: String,
//     pub report_sended: bool
// }


impl NewPacketInfoTrait for NewPacketInfo{}
pub trait NewPacketInfoTrait
{
    fn from_err(err: &str, packet_name: &str, task: &Task) -> NewPacketInfo
    {
        NewPacketInfo::new_err(packet_name, settings::Date::now().as_serialized().as_str(), task, err)
    }
    fn from_packet(packet: &Packet, task: &Task) -> NewPacketInfo
    {
        NewPacketInfo::new_doc(packet.get_packet_name(), settings::Date::now().as_serialized().as_str(), task, &packet.as_doc())
    }
    fn not_packet(packet_dir: &str, task: &Task) -> NewPacketInfo
    {
        NewPacketInfo::new_empty(packet_dir, settings::Date::now().as_serialized().as_str(), task)
    }
}