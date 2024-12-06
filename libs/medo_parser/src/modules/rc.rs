use std::path::PathBuf;
use quick_xml::DeError;
use serde::{Serialize, Deserialize};
use crate::{MedoParser, MedoParserError, open_file, FileEncoding};


#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
#[serde(rename="Regcard")]
///Структура для содержимого файла .rc
pub struct RcParser
{
    ///<barcode>2100088686184</barcode>
    #[serde(skip_serializing_if="Option::is_none")]
    pub barcode: Option<String>,
    ///<change_time>05.12.2024 22:45:38</change_time>
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_time: Option<String>,
    ///Составное название с реквизитами  
    /// <content>Указ Президента Российской Федерации  от  10.10.2024  №  999
    ///&quot;ааа ббб ввв &quot; </content>
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,
    ///Только название  
    /// <content_2>&quot;ааа ббб ввв&quot;</content_2>
    #[serde(skip_serializing_if="Option::is_none")]
    pub content_2: Option<String>,
    ///исполнитель  
    #[serde(skip_serializing_if="Option::is_none")]
    pub executor: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub guid: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///незнаю, та же что и дата подписания но со временем
    /// 30.05.2022 13:59:34
    pub publ_date: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///дата подписания
    pub regdate: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Номер документа
    pub regnumber: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///ФИО подписанта
    pub signer_name: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Должность подписанта
    pub signer_org: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Например "Открытый"
    pub status: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Вид документа
    pub viddoc: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Количество страний
    pub pages_orig: Option<u32>,
}

impl MedoParser for RcParser
{
    const EXTENSION : &'static str = "rc";
    fn parse(file: &PathBuf, _paths: Option<&mut Vec<PathBuf>>) -> Result<Self, MedoParserError> 
    {
        let decoded = open_file(file, Some(FileEncoding::Windows1251))?;
        let de: RcParser = quick_xml::de::from_reader(decoded.1.as_bytes())?;
        Ok(de)
    }
}
