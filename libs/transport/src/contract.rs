use std::fmt::Display;
use serde::{Deserialize, Serialize};
use settings::Task;
use crate::NewPacketInfo;
impl service::Converter for Contract{}

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
}

pub trait FromWsCommand
{
    fn get_command(&self) -> Contract;
}