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


// pub trait ContractBuilder
// {
//     fn new_packet(packet: NewPacketInfo)-> Contract
//     {
//         Contract::NewPacket(packet)
//     }
//     fn new_error(error: String)-> Contract
//     {
//         Contract::Error(error)
//     }
//     fn new_task_updated(task: &Task)-> Contract
//     {
//         Contract::TaskUpdated(task.clone())
//     }
//     fn new_task_deleted(task: &Task)-> Contract
//     {
//         Contract::TaskDeleted(task.clone())
//     }    
// }

// impl ContractBuilder for Contract{}
