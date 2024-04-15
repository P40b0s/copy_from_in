use std::fmt::Display;

use serde::{Deserialize, Serialize};
use service::WebsocketMessage;
use settings::Task;

use crate::NewPacketInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Contract
{
    TaskUpdated(Task),
    TaskDeleted(Task),
    NewPacket(NewPacketInfo),
    Error(String),
    ErrorConversion(String)
}


impl Contract
{
    pub fn new_packet(packet: NewPacketInfo)-> Self
    {
        Self::NewPacket(packet)
    }
    pub fn new_error(error: String)-> Self
    {
        Self::Error(error)
    }
    pub fn new_task_updated(task: &Task)-> Self
    {
        Self::TaskUpdated(task.clone())
    }
    pub fn new_task_deleted(task: &Task)-> Self
    {
        Self::TaskDeleted(task.clone())
    }
    pub fn as_ws_message(self) -> WebsocketMessage
    {
        self.into()
    }
}

impl From<WebsocketMessage> for Contract
{
    fn from(value: WebsocketMessage) -> Self 
    {
        let obj = value.extract_payload::<Contract>();
        if let Ok(msg) = obj
        {
            msg
        }
        else 
        {
            let error = obj.err().unwrap().to_string();
            logger::error!("{}", &error);
            Contract::ErrorConversion(error)
        }
    }
}


impl From<Contract> for WebsocketMessage
{
    fn from(value: Contract) -> Self 
    {

        if let Contract::Error(e) = &value
        {
            WebsocketMessage::new_error("error", e)
        }
        else
        {
            WebsocketMessage::new("CMD", &value)
        }
    }
}



pub trait FromWsCommand
{
    fn get_command(&self) -> Contract;
}