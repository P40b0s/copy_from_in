use std::{marker::PhantomData, path::Path};

use logger::debug;
use medo_parser::PacketInfo;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use settings::Task;
use utilites::Date;
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CopyStatus
{
    pub copy_ok: bool,
    pub copy_path: String
}
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
    pub report_sended: bool,
    pub copy_status: Vec<CopyStatus>
}

impl Packet
{
    /// В этом методе дополнительно считаем хэш pdf файла  
    /// основной метод прозрачно пробрасывается в  
    /// `medo_parser::PacketInfo::parse()` -> `medo_parser::Packet::parse()`
    /// меняем, сделал футуру чтобы весь парсер с собой не таскать а только модель, поэтому от прозрачного проброса пришлось отказаться  
    /// запускаем отдельно и передаем сюда
    /// если packet_info не является пакетов возвращаем None
    pub fn parse<P: AsRef<Path>>(path: P, packet_info: PacketInfo, task: &Task) -> Self
    {
        let path = Path::new(path.as_ref());
        let mut packet_info = packet_info;
        if packet_info.default_pdf.is_some()
        {
            let path = Path::new(path).join(packet_info.default_pdf.as_ref().unwrap());
            packet_info.pdf_hash = utilites::Hasher::hash_from_path(path);
        }
        //Добавил добавление source_giud при парсинге
        if let Some(si) = packet_info.sender_info.as_ref()
        {
            packet_info.sender_id = si.source_guid.clone()
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
            copy_status: Vec::new()
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
            copy_status: Vec::new()
        }
    }
    pub fn new_err<S: ToString>(name: S, task: &Task, error: S) -> Self
    {

        let mut pi = PacketInfo::default();
        pi.packet_directory = name.to_string();
        pi.delivery_time = Self::time_now();
        pi.add_error(error);
        Self
        {
            id: Self::id(task.get_task_name(), &pi.packet_directory),
            name: name.to_string(),
            parse_time: Self::time_now(),
            report_sended: false,
            task: task.clone(),
            packet_info: pi,
            copy_status: Vec::new()
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
            copy_status: Vec::new()
        }
    }
    pub fn new_from_db<S: ToString>(task: Task, id: S, packet: &PacketInfo, report_sended: bool, copy_status: Vec<CopyStatus>) -> Self
    {
        Self
        {
            id: id.to_string(),
            name: packet.packet_directory.clone(),
            parse_time: packet.delivery_time.clone(),
            report_sended,
            task,
            packet_info: packet.clone(),
            copy_status
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
    pub fn get_error(&self) -> Option<&String>
    {
        self.packet_info.get_error()
    }
    pub fn add_copy_status(&mut self, is_copied: bool, path: String)
    {
        self.copy_status.push(
            CopyStatus 
            { 
                copy_ok: is_copied,
                copy_path: path
            }
        );
    }
    ///Все файлы успешно скопированы
    pub fn copy_ok(&self) -> bool
    {
        self.copy_status.iter().all(|a| a.copy_ok)
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


#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
struct Metainfo<T> 
{
    info: String,
    result: Result<T>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
struct Result<T>
{
    #[serde(alias = "obj1", alias="obj2")]
    object: T
}

#[test]
fn test()
{
    // let v = Metainfo
    // {
    //     info: "ASd".to_owned(),
    //     result: Result
    //     {
    //         object: ContactType {
    //             contact_type: "asd".to_owned(),
    //             value: "41234123123123".to_owned()
    //         }
    //     }
    // };
    // let s = serde_json::to_string_pretty(&v).unwrap();
    // println!("{}", &s);
    let js = r#"{
        "info": "ASd",
        "result": {
          "obj": {
            "contact_type": "asd",
            "value": "41234123123123"
          }
        }
      }"#;
    let des = serde_json::from_str::<Metainfo<ContactType>>(js).unwrap();
   
}

//where for <'de> T : Deserialize<'de>, for <'de> Self : Deserialize<'de>