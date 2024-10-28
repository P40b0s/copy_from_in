use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct File
{   
    pub file_name: String,
    pub file_type: String,
    pub path: String
}

impl File
{
    pub fn extension(&self) -> &str
    {
        &self.file_type
    }
    pub fn name(&self) -> &str
    {
        &self.file_name
    }
    pub fn path(&self) -> &str
    {
        &self.path
    }
}

/// Структура для запроса страницы файла или всего файла из API
#[derive(Deserialize, Serialize, Clone)]
pub struct FileRequest
{
    pub file: File,
    pub page_number: Option<u32>,
}
///
#[derive(Deserialize, Serialize, Clone)]
pub struct FilesRequest
{
    pub task_name: String,
    pub dir_name: String
}
