use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum MedoParserError
{   
    #[error("`{0}`")]
    Io(#[from] std::io::Error),
    #[error("`{0}`")]
    ParseError(String),
    #[error("При парсинге `{0}` небыл передан обязательный для xml парсера агрумент `paths`")]
    ParserPathError(String),
    #[error("`{0}`")]
    UnzipError(String),
    #[error("`{0}`")]
    ZipEmpty(String),
    #[error("`{0}`")]
    PacketError(String),
    #[error("Пакет `{0}` не является допустимым транспортным пакетом")]
    IsNotPacketError(String),
    #[error("Ошибка обработки файла .ltr: `{0}`")]
    LtrError(String),
    #[error("Ошибка десериализации: `{0}`")]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    XmlError(#[from] quick_xml::DeError),

}


impl Serialize for MedoParserError 
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
    S: serde::Serializer 
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}


// impl From<Option<quick_xml::DeError>> for MedoParserError
// {
//     fn from(value: Option<quick_xml::DeError>) -> Self 
//     {
//         match value 
//         {
//             Some(val) =>
//             {
//                 match val
//                 {
//                     quick_xml::DeError::Custom(c) =>
//                     {
//                         let c = c.replace("missing field", "отсуствует поле");
//                         MedoParserError::SerdeError(format!("Ошибка десериализации: {}", c.to_string()))
//                     },
//                     _ => MedoParserError::SerdeError(format!("Ошибка десериализации: {}", val.to_string()))
//                 }
//             },
//             None => MedoParserError::None
//         }
//     }
// }

// impl serde::de::Error for quick_xml::DeError 
// {
//     fn custom<T: fmt::Display>(msg: T) -> Self {
//         quick_xml::DeError::Custom(format!("123321123321{}", msg.to_string()))
//     }
// }

