use std::{fmt::{self, Display}};

use serde::de;

#[derive(Debug, Clone)]
pub enum MedoParserError
{   
    ParseError(String),
    SerdeError(String),
    None
}

pub type Result<T> = std::result::Result<T, MedoParserError>;
impl std::error::Error for MedoParserError {}

impl de::Error for MedoParserError
{
    fn custom<T: Display>(msg: T) -> Self 
    {
        MedoParserError::ParseError(msg.to_string())
    }
}
impl fmt::Display for MedoParserError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match self 
        {
            MedoParserError::ParseError(p) => write!(f, "{}", p),
            MedoParserError::SerdeError(p) => write!(f, "{}", p),
            MedoParserError::None =>  Ok(()),
        }
    }
}

impl From<serde_json::Error> for MedoParserError
{
    fn from(error: serde_json::Error) -> Self 
    {
        MedoParserError::SerdeError(format!("{} {} {} {}",error.to_string(), error.is_data(), error.column(), error.line()))
    }
}

impl From<quick_xml::DeError> for MedoParserError
{
    fn from(value: quick_xml::DeError) -> Self 
    {
        match value 
        {
            quick_xml::DeError::Custom(c) =>
            {
                let c = c.replace("missing field", "отсуствует поле");
                MedoParserError::SerdeError(format!("Ошибка десериализации: {}", c.to_string()))
            },
            _ => MedoParserError::SerdeError(format!("Ошибка десериализации: {}", value.to_string()))
        }
       
    }
}
impl From<Option<quick_xml::DeError>> for MedoParserError
{
    fn from(value: Option<quick_xml::DeError>) -> Self 
    {
        match value 
        {
            Some(val) =>
            {
                match val
                {
                    quick_xml::DeError::Custom(c) =>
                    {
                        let c = c.replace("missing field", "отсуствует поле");
                        MedoParserError::SerdeError(format!("Ошибка десериализации: {}", c.to_string()))
                    },
                    _ => MedoParserError::SerdeError(format!("Ошибка десериализации: {}", val.to_string()))
                }
            },
            None => MedoParserError::None
        }
    }
}

// impl serde::de::Error for quick_xml::DeError 
// {
//     fn custom<T: fmt::Display>(msg: T) -> Self {
//         quick_xml::DeError::Custom(format!("123321123321{}", msg.to_string()))
//     }
// }

