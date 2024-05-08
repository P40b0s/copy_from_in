use std::{borrow::Cow, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};
use utilites::{Date, DateFormat};
use crate::{get_entries, helpers::DatesHelper, traits::Uid, Container, MedoParser, MedoParserError, RcParser, XmlParser};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PacketError
{
    None,
    Error(String)
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Packet
{
    #[serde(skip_serializing_if="Option::is_none")]
    xml: Option<XmlParser>,
    #[serde(skip_serializing_if="Option::is_none")]
    rc: Option<RcParser>,
    #[serde(skip_serializing_if="Option::is_none")]
    founded_files: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    packet_dir: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    packet_date_time: Option<String>,
    wrong_encoding: bool,
    error: PacketError,
    #[serde(skip_serializing)]
    path: Option<PathBuf>
}

impl Packet
{
    pub fn wrong_encoding(&self) -> bool
    {
        self.wrong_encoding
    }
    fn add_error(&mut self, error: MedoParserError)
    {
        self.error =  PacketError::Error(error.to_string());
    }
    pub fn get_error(&self) -> Option<Cow<str>>
    {
        match &self.error
        {
            PacketError::Error(e) => Some(Cow::from(e)),
            PacketError::None => None
        }
    }
    ///Проверка распарсился ли транспоттный пакет
    pub fn is_parsed(&self) -> bool
    {
        self.xml.is_some() || self.rc.is_some()
    }
    pub fn get_xml(&self) -> Option<&XmlParser>
    {
        self.xml.as_ref()
    }
    pub fn get_container(&self) -> Option<&Container>
    {
        self.xml.as_ref().and_then(|x|x.container.as_ref())
    }
    pub fn get_rc(&self) -> Option<&RcParser>
    {
        self.rc.as_ref()
    }
    pub fn get_packet_name(&self) -> &str
    {
        self.packet_dir.as_ref().map_or("", |f| f.as_str())
    }
    pub fn get_packet_files(&self) -> Option<&Vec<String>>
    {
        self.founded_files.as_ref()
    }
    ///Получение организации из поля source
    pub fn get_organization(&self) -> Option<Cow<'_, str>>
    {
        let org = self.get_xml().and_then(|x| x.get_organization());
        org
    }
     ///Получение организации из поля source
    pub fn get_document_date_number(&self) -> Option<super::xml::Number>
    {
        let num1 = self.get_xml().and_then(|x| x.get_document().and_then(|d| d.num.clone()));
        let num2 = self.get_container()
        .and_then(|c| c.authors.authors.first()
            .and_then(|a| Some( super::xml::Number { number: a.registration.number.clone(), date: a.registration.date.clone()})));
        num1.or(num2)
    }
    ///получение вида документа из поля kind
    pub fn get_document_type(&self) -> Option<String>
    {
        let org = self.get_xml().and_then(|x| x.get_document().and_then(|d| d.kind.clone()));
        let org2 = self.get_container().and_then(|c| Some(c.requisites.document_kind.clone()));
        org.or(org2)
    }
    ///получение уникального идентификатора документа
    pub fn get_document_uid(&self) -> Option<String>
    {
        let org = self.get_xml().and_then(|x| x.get_document().and_then(|d| Some(d.get_uid().into_owned())));
        let org2 = self.get_container().and_then(|c| Some(c.get_uid().into_owned()));
        org.or(org2)
    }
    pub fn get_source_addressee(&self) -> Option<String>
    {
        self.get_xml().and_then(|a| a.get_medo_addressee().and_then(|m| Some(m.into_owned())))
    }
    pub fn get_packet_date_time(&self) -> Option<Cow<str>>
    {
        let dt = self.packet_date_time.as_ref();
        match dt
        {
            Some(s) => Some(Cow::from(s)),
            None => None
        }
    }
    pub fn get_packet_type(&self) -> Option<Cow<str>>
    {
        self.get_xml().and_then(|x|x.get_header().and_then(|h|Some(h.get_type())))
    }
    pub fn get_source_uid(&self) -> Option<Cow<str>>
    {
        self.get_xml().and_then(|x|x.get_header().and_then(|h|Some(h.get_source().get_uid())))
    }

    pub fn parse<P: AsRef<Path>>(path: P) -> Self
    {
        let mut p = Packet 
        {
            xml: None,
            rc: None,
            founded_files: None,
            wrong_encoding: false,
            error: PacketError::None,
            packet_dir: None,
            packet_date_time: None,
            path: Some(path.as_ref().into())
        };
        let result = p.parse_transport_packet();
        if result.is_err()
        {
            p.add_error(result.err().unwrap());
            return p;
        }
        let r = result.unwrap();
        p.xml = r.xml;
        p.founded_files = r.founded_files;
        p.wrong_encoding = r.wrong_encoding;
        p.packet_dir = r.packet_dir;
        p.path = r.path;
        p
    }
  
    fn parse_transport_packet(&mut self) -> Result<Self, MedoParserError>
    {
        let mut paths: Vec<PathBuf>  = vec![];
        let mut base_dir = Some(String::new());
        if let Some(d) = self.path.as_ref().unwrap().into_iter().last()
        {
            if let Some(d) = d.to_str()
            {
                base_dir = Some(d.to_owned());
            }
        }
        if base_dir.is_none()
        {
            return Err(MedoParserError::ParseError(format!("Ошибка определения базовой директории пакета {}", self.path.as_ref().unwrap().display())));
        }
        if let Ok(created) = self.path.as_ref().unwrap().metadata().and_then(|m|m.created())
        {
            self.packet_date_time = Some(Date::from_system_time(created).format(DateFormat::Serialize));
        }

        let base_dir = base_dir.unwrap();
        self.packet_dir = Some(base_dir.clone());
        //let mut comm: Communication = Communication::default();
        let mut file_count = 0;
        if let Some(files) = get_entries(self.path.as_ref().unwrap())
        {
            if files.len() == 0
            {
                return Err(MedoParserError::ParseError(format!("Ошибка, в транспотрном пакете {} отсутсвуют файлы", self.path.as_ref().unwrap().display())));
            }
            file_count = 0;
            //Добавляем все файлы виз директории в список, добавляем отдельно потому что если будет ошибка то в этот список попадут не все файлы
            files.iter().for_each(|f| 
            {
                if f.path().is_file()
                {
                    if let Some(file) = f.path().file_name().and_then(|fl| fl.to_str())
                    {
    
                        paths.push(PathBuf::from(file));
                    }
                }
            });
            //Собираем все имена найденных файлов
            self.founded_files =  Some(paths.iter().map(|f|f.display().to_string()).collect());
            for f in files
            {
                if let Some(ext) = f.path().extension()
                {
                    if let Some(ext) = ext.to_str()
                    {
                        //let p = PathBuf::from(f.path().file_name().unwrap());
                        //paths.push(p);
                        match ext
                        {
                            XmlParser::EXTENSION =>
                            {
                                let xml = XmlParser::parse(&f.path(), Some(&mut paths))?;
                                self.xml = Some(xml);
                            },
                            RcParser::EXTENSION =>
                            {
                                let rc = RcParser::parse(&f.path(), None)?;
                                self.rc = Some(rc);
                            }
                            _ => {}
                        };
                        file_count = file_count + 1;
                    }
                }
            }
        }
        //Если не вылетело с ошибкой то заново формируем лист файлов, возможно добавились файлы из архива
        self.founded_files =  Some(paths.iter().map(|f|f.display().to_string()).collect());
        //rc файл заменяет собой пакет мэдо, так что либо то либо то
        if !self.is_parsed()
        {
            if file_count > 0
            {
                return Err(MedoParserError::ParseError(format!("Ошибка обработки транспотртного пакета {}, текущей парсер не может обрабатывать поступившие файлы, необходимо обратиться к администратору", self.path.as_ref().unwrap().display())));
            }
            if file_count == 0
            {
                return Err(MedoParserError::ParseError(format!("Ошибка обработки транспотрного пакета {}, в текущей директории отсутсвуют файлы (есть директории), необходимо обратиться к администратору", self.path.as_ref().unwrap().display())));
            }
            return Err(MedoParserError::ParseError(format!("Ошибка обработки транспотрного пакета {}, необходимо обратиться к администратору", self.path.as_ref().unwrap().display())));
        }
        Ok(self.clone())
    }
}