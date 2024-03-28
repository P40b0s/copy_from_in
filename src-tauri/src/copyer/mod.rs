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
    pub doc_type: Option<String>,
    pub number: Option<String>,
    pub sign_date: Option<String>,
    pub name: String,
    pub parse_time: String
}
impl NewDocument
{
    pub fn new(packet_name: &str) -> Self
    {
        Self
        {
            organization: None,
            doc_type: None,
            number: None,
            sign_date: None,
            name: packet_name.to_owned(),
            parse_time: settings::Date::now().as_serialized()
        }
    }
}

impl From<&Packet> for NewDocument
{
    fn from(value: &Packet) -> Self 
    {
        let organization = value.get_organization().map_or( None, |o| Some(o.into_owned()));
        let date_number = value.get_document_date_number();
        let date = date_number.as_ref().and_then(|d| d.date.clone());
        let number = date_number.as_ref().and_then(|d| d.number.clone());
        Self
        {
            organization,
            doc_type: value.get_document_type(),
            number,
            sign_date: date,
            name: value.get_packet_name().to_owned(),
            parse_time: settings::Date::now().as_serialized()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewPacketInfo
{
    document: Option<NewDocument>,
    error: Option<String>,
    task: Option<Task>
}
impl NewPacketInfo
{
    pub fn get_packet_name(&self) -> &str
    {
        if let Some(d) = self.document.as_ref()
        {
            &d.name
        }
        else
        {
            self.error.as_ref().unwrap()
        }
    }
}

impl From<&Vec<ValidationError>> for NewPacketInfo
{
    fn from(value: &Vec<ValidationError>) -> Self 
    {
        let mut errors = String::new();
        let error = value.iter().fold(&mut errors, |acc, val|
        {
            let str = [val.to_string(), "\\n".to_owned()].concat();
            acc.push_str(&str);
            acc
        });
        Self
        {
            document: None,
            error: Some(error.clone()),
            task: None
        }
    }
}

impl From<&Packet> for NewPacketInfo
{
    fn from(value: &Packet) -> Self 
    {
        Self
        {
            document: Some(value.into()),
            error: None,
            task: None
        }
    }
}
impl From<NewDocument> for NewPacketInfo
{
    fn from(value: NewDocument) -> Self 
    {
        Self
        {
            document: Some(value),
            error: None,
            task: None
        }
    }
}

impl From<&NewDocument> for NewPacketInfo
{
    fn from(value: &NewDocument) -> Self 
    {
        Self
        {
            document: Some(value.to_owned()),
            error: None,
            task: None
        }
    }
}

impl From<String> for NewPacketInfo
{
    fn from(value: String) -> Self 
    {
        Self
        {
            document: None,
            error: Some(value),
            task: None
        }
    }
}