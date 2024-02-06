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
    #[serde(skip_serializing_if="Option::is_none")]
    pub barcode: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub change_time: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Составное название с реквизитами
    pub content: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Только название
    pub content_2: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///исполнитель
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
        let de: Result<RcParser, DeError> = quick_xml::de::from_reader(decoded.1.as_bytes());
        if de.is_err()
        {
            return Err(MedoParserError::SerdeError(format!("{}, {}", file.display(), de.err().unwrap())));
        }
        Ok(de.unwrap())
    }
}
