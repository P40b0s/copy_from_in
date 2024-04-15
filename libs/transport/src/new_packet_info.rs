use serde::{Deserialize, Serialize};
use settings::Task;

use crate::new_document::NewDocument;

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
    pub fn new_doc<S: ToString>(name: S, parse_time: S, task: &Task, document: &NewDocument) -> Self
    {
        Self
        {
            name: name.to_string(),
            parse_time: parse_time.to_string(),
            report_sended: false,
            task: task.clone(),
            document: Some(document.clone()),
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
            document: None,
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
            document: None,
            error: None
        }
    }
    pub fn get_task(&self) -> &Task
    {
        &self.task
    }
    pub fn get_document(&self) -> &Option<NewDocument>
    {
        &self.document
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