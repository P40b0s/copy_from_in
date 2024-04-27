use std::path::Path;

use logger::debug;
use medo_parser::PacketInfo;
use serde::{Deserialize, Serialize};
use settings::Task;

///Эта структура будет ходить между клиентом - сервером
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Packet
{
    packet_info: Option<PacketInfo>,
    error: Option<String>,
    task: Task,
    pub name: String,
    pub parse_time: String,
    pub report_sended: bool
}

impl Packet
{
    pub fn parse<P: AsRef<Path>>(path: P, task: &Task) -> Self
    {
        let packet_info = PacketInfo::parse(path);
        let name = packet_info.packet_directory.clone();
        let parse_time = packet_info.delivery_time.clone();
        if let Some(e) = packet_info.error.clone()
        {
            debug!("Ошибка парсинга пакета {} -> {:?}", &e, &packet_info);
            return Self
            {
                name,
                parse_time,
                report_sended: false,
                task: task.clone(),
                packet_info: None,
                error: Some(e)
            };
        }
        else 
        {
            return Self
            {
                name,
                parse_time,
                report_sended: false,
                task: task.clone(),
                packet_info: Some(packet_info),
                error: None
            };
        }
       
        
    }
    pub fn new_packet(task: &Task, packet: PacketInfo) -> Self
    {
        let name = packet.packet_directory.clone();
        let parse_time = packet.delivery_time.clone();
        Self
        {
            name,
            parse_time,
            report_sended: false,
            task: task.clone(),
            packet_info: Some(packet),
            error: None
        }
    }
    pub fn new_err<S: ToString>(name: S, parse_time: S, task: &Task, error: S) -> Self
    {
        Self
        {
            name: name.to_string(),
            parse_time: parse_time.to_string(),
            report_sended: false,
            task: task.clone(),
            packet_info: None,
            error: Some(error.to_string())
        }
    }
    pub fn new_empty<S: ToString>(name: S, parse_time: S, task: &Task) -> Self
    {
        Self
        {
            name: name.to_string(),
            parse_time: parse_time.to_string(),
            report_sended: false,
            task: task.clone(),
            packet_info: None,
            error: None
        }
    }
    pub fn get_task(&self) -> &Task
    {
        &self.task
    }
    pub fn get_packet_info(&self) -> &Option<PacketInfo>
    {
        &self.packet_info
    }
    pub fn get_error(&self) -> &Option<String>
    {
        &self.error
    }
    pub fn get_packet_name(&self) -> &str
    {
        &self.name
    }
    pub fn get_parse_time(&self) -> &str
    {
        &self.parse_time
    }
}