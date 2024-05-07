use std::path::Path;

use logger::debug;
use medo_parser::PacketInfo;
use serde::{Deserialize, Serialize};
use settings::Task;
use utilites::Date;

///Эта структура будет ходить между клиентом - сервером
/// все ошибки будут внутри packet_info.error
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Packet
{
    packet_info: PacketInfo,
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
        Self
        {
            name,
            parse_time,
            report_sended: false,
            task: task.clone(),
            packet_info: packet_info,
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
            packet_info: packet,
        }
    }
    pub fn new_err<S: ToString>(name: S, task: &Task, error: S) -> Self
    {

        let mut pi = PacketInfo::default();
        pi.packet_directory = name.to_string();
        pi.delivery_time = Self::time_now();
        pi.error = Some(error.to_string());
        Self
        {
            name: name.to_string(),
            parse_time: Self::time_now(),
            report_sended: false,
            task: task.clone(),
            packet_info: pi,
        }
    }
    fn time_now() -> String
    {
        Date::now().format(utilites::DateFormat::Serialize)
    }
    pub fn new_empty<S: ToString>(name: S, task: &Task) -> Self
    {
        let mut pi = PacketInfo::default();
        pi.packet_directory = name.to_string();
        pi.delivery_time = Self::time_now();
        Self
        {
            name: name.to_string(),
            parse_time: Self::time_now(),
            report_sended: false,
            task: task.clone(),
            packet_info: pi,
        }
    }
    pub fn get_task(&self) -> &Task
    {
        &self.task
    }
    pub fn get_packet_info(&self) -> &PacketInfo
    {
        &self.packet_info
    }
    pub fn get_error(&self) -> &Option<String>
    {
        &self.packet_info.error
    }
    pub fn is_err(&self) -> bool
    {
        self.packet_info.error.is_some()
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