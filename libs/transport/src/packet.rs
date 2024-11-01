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
    id: String,
    packet_info: PacketInfo,
    task: Task,
    pub name: String,
    pub parse_time: String,
    pub report_sended: bool
}

impl Packet
{
    /// В этом методе дополнительно считаем хэш pdf файла  
    /// основной метод прозрачно пробрасывается в  
    /// `medo_parser::PacketInfo::parse()` -> `medo_parser::Packet::parse()`
    /// меняем, сделал футуру чтобы весь парсер с собой не таскать а только модель, поэтому от прозрачного проброса пришлось отказаться  
    /// запускаем отдельно и передаем сюда
    pub fn parse<P: AsRef<Path>>(path: P, packet_info: PacketInfo, task: &Task) -> Self
    {
        let path = Path::new(path.as_ref());
        let mut packet_info = packet_info;
        if packet_info.default_pdf.is_some()
        {
            let path = Path::new(path).join(packet_info.default_pdf.as_ref().unwrap());
            packet_info.pdf_hash = utilites::Hasher::hash_from_path(path);
        }
        let name = packet_info.packet_directory.clone();
        let parse_time = packet_info.delivery_time.clone();
        Self
        {
            id: Self::id(task.get_task_name(), &packet_info.packet_directory),
            name,
            parse_time,
            report_sended: false,
            task: task.clone(),
            packet_info,
        }
    }
    pub fn new_packet(task: &Task, packet: PacketInfo) -> Self
    {
        let name = packet.packet_directory.clone();
        let parse_time = packet.delivery_time.clone();
        Self
        {
            id: Self::id(task.get_task_name(), &packet.packet_directory),
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
            id: Self::id(task.get_task_name(), &pi.packet_directory),
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
    fn id<S: AsRef<str>>(task_name: S, packet_dir: S) -> String
    {
        utilites::Hasher::hash_from_strings(&[task_name, packet_dir])
    }
    pub fn new_empty<S: ToString>(name: S, task: &Task) -> Self
    {
        let mut pi = PacketInfo::default();
        pi.packet_directory = name.to_string();
        pi.delivery_time = Self::time_now();
        Self
        {
            id: Self::id(task.get_task_name(), &pi.packet_directory),
            name: name.to_string(),
            parse_time: Self::time_now(),
            report_sended: false,
            task: task.clone(),
            packet_info: pi,
        }
    }
    pub fn new_from_db<S: ToString>(task: Task, id: S, packet: &PacketInfo, report_sended: bool) -> Self
    {
        Self
        {
            id: id.to_string(),
            name: packet.packet_directory.clone(),
            parse_time: packet.delivery_time.clone(),
            report_sended,
            task,
            packet_info: packet.clone(),
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
    pub fn get_id(&self) -> &str
    {
        &self.id
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactType
{
    pub contact_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactInfo
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub person: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub post: Option<String>,
    pub contacts: Vec<ContactType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Поле введено отдельно, в него информация при парсинге не поступает, это для фронта
    pub note: Option<String>
}
impl Default for ContactInfo
{
    fn default() -> Self 
    {
        ContactInfo 
        { 
            id: None,
            organization: None,
            person: None,
            post: None,
            contacts: vec![],
            photo: None,
            note: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Senders
{
    pub id: String,
    pub organization: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub medo_addresse: Option<String>,
    pub contact_info: Vec<ContactInfo>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon: Option<String>,
}
