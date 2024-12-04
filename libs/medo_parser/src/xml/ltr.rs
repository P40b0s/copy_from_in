use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::{MedoParserError, open_file, FileEncoding};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
///Структура для содержимого файла .ltr
pub struct Ltr
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_autosend: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_esd: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_delivered: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub is_readed: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub annotation_file: Option<String>,
    pub addresses: Vec<String>,
    pub files: Vec<String>,
}

impl Ltr
{
    pub fn parse_file(file: &PathBuf) -> Result<Ltr, crate::MedoParserError> 
    {
        let decoded = open_file(file, Some(FileEncoding::Windows1251))?;
        let ltr = Self::parse_string(&decoded.1)?;
        Ok(ltr)
    }
    pub fn parse_string(data: &str) -> Result<Ltr, crate::MedoParserError> 
    {
        let mut ltr = Ltr::default();
        let mut is_addr = false;
        let mut is_file = false;
        for line in data.lines()
        {
            match line 
            {
                s if s.starts_with("ФАЙЛ=") => ltr.annotation_file = s.strip_prefix("ФАЙЛ=").and_then(|s| Some(s.to_owned())),
                s if s.starts_with("ДАТА=") => ltr.date = s.strip_prefix("ДАТА=").and_then(|s| Some(s.to_owned())),
                s if s.starts_with("ТЕМА=") => ltr.theme = s.strip_prefix("ТЕМА=").and_then(|s| Some(s.to_owned())),
                s if s.starts_with("АВТООТПРАВКА=") => ltr.is_autosend = s.strip_prefix("АВТООТПРАВКА=").and_then(|s| Self::num_to_bool(s)),
                s if s.starts_with("ЭЦП=") => ltr.is_esd = s.strip_prefix("ЭЦП=").and_then(|s| Self::num_to_bool(s)),
                s if s.starts_with("ДОСТАВЛЕНО=") => ltr.is_delivered = s.strip_prefix("ДОСТАВЛЕНО=").and_then(|s| Self::num_to_bool(s)),
                s if s.starts_with("ПРОЧТЕНО=") => ltr.is_readed = s.strip_prefix("ПРОЧТЕНО=").and_then(|s| Self::num_to_bool(s)),
                s if s.starts_with("[ФАЙЛЫ]") => 
                {
                    is_addr = false;
                    is_file = true;
                },
                s if s.starts_with("[АДРЕСАТЫ]") => 
                {
                    is_addr = true;
                    is_file = false;
                },
                s if s.len() > 0 && (b'0'..b'9').contains(&s.as_bytes()[0]) =>
                {
                    
                    logger::debug!("{}", s);
                    if is_addr
                    {
                        ltr.addresses.push(s[2..].to_owned());
                    }
                    if is_file
                    {
                        ltr.files.push(s[2..].to_owned());
                    }
                },
                _ => 
                {
                    is_addr = false;
                    is_file = false;
                }
            }
        }
        if ltr.addresses.is_empty()
        {
            return Err(MedoParserError::LtrError("В файле не найден ни один адрес МЭДО".to_owned()));
        }
        Ok(ltr)
    }
    fn num_to_bool(num: &str) -> Option<bool>
    {
        let n  = num.parse::<u8>();
        match n
        {
            Ok(n) if n == 0 => Some(false),
            Ok(n) if n == 1 => Some(true),
            _ => None
        }
    }
}
impl Default for Ltr
{
    fn default() -> Self 
    {
        Ltr 
        {
            theme: None,
            is_autosend: None,
            is_esd: None,
            is_delivered: None,
            is_readed:None,
            date: None,
            annotation_file: None,
            addresses: vec![],
            files: vec![]
        }
    }
}

#[cfg(test)]
mod test
{
    use logger::debug;

    use super::Ltr;

    const LTR: &'static str = "[ПИСЬМО КП ПС СЗИ]
ТЕМА=ЭСД МЭДО (767-р от 30.03.2023 {F19AF9F1-84CA-47A7-BC1C-A1CD4CFCB0BB})
АВТООТПРАВКА=1
ЭЦП=0
ДОСТАВЛЕНО=1
ПРОЧТЕНО=1
ДАТА=30.03.2023 16:16:52
[АДРЕСАТЫ]
0=WH-MEDO~APRF
[ФАЙЛЫ]
0=text0000000000.pdf
1=document.xml
[ТЕКСТ]
ФАЙЛ=annotation.txt
";
    #[test]
    fn test_parse_str()
    {
        let ltr = Ltr::parse_string(LTR).unwrap();
        assert!(ltr.files[1] == "document.xml".to_owned());
    }
}