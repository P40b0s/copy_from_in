use std::{path::{PathBuf, Path}, borrow::Cow};
use logger::backtrace;
use quick_xml::DeError;
use serde::{Serialize, Deserialize};
use super::Ltr;
use crate::{Communication, MedoParserError, Container, extension_path_is, FileEncoding, cleary_xml_namespaces, open_file, MedoParser, get_entries, Document, Source, Header, Acknowledgment};

impl MedoParser for XmlParser
{
    const EXTENSION : &'static str = "xml";
    fn parse(file: &PathBuf, paths: Option<&mut Vec<PathBuf>>) -> Result<Self, crate::MedoParserError> 
    {
        if let Some(paths) = paths
        {
            let result: (Communication, Option<Ltr>, Option<Container>, bool) = process_root_xml(file, paths)?;
            Ok(Self{ communication: result.0, ltr: result.1, container: result.2, wrong_encoding: result.3 })
        }
        else
        {
            return Err(MedoParserError::ParserPathError(file.display().to_string()));
        }
    }
}

///Основкная стратегия парсинга пакета:
///В корне лежит сопроводиловка в виде xml файла, его мы парсим через process_root_xml
///Так же в корне лежит файл ltr из готорого мы можем узнать мэдо адрес отправителя данного пакета, его тоже парсим
///Далее если версия пакета выше 2.7 то у него есть поле в котором указан зип файл с вложенным в него документом и еще одним сопроводительным файлом
///обычно passport.xml, разархивируем пакет, парсим его и добавляем все файлы в вектор paths 
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct XmlParser
{
    pub communication: Communication,
    #[serde(skip_serializing_if="Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if="Option::is_none")]
    ltr: Option<Ltr>,
    #[serde(skip_serializing)]
    pub wrong_encoding: bool,
}
impl XmlParser
{
    pub fn is_container(&self) -> bool
    {
        self.container.is_some()
    }
    pub fn get_medo_addressee(&self) -> Option<Cow<str>>
    {
        if let Some(ltr) = self.ltr.as_ref()
        {
            if !ltr.addresses.is_empty()
            {
                let first = ltr.addresses.first().unwrap();
                return Some(Cow::from(first));
            }
        }
        None
    }
    pub fn is_acknowledgment(&self) -> bool
    {
        self.communication.acknowledgment.is_some()
    }
    pub fn get_version(&self) -> Cow<str>
    {
        Cow::from(&self.communication.version)
    }
    pub fn get_organization(&self) -> Option<Cow<str>>
    {
        match self.get_source()
        {
            Some(s) => Some(s.get_organization()),
            None=> None
        }
    }
    pub fn get_header(&self) -> Option<&Header>
    {
        self.communication.header.as_ref()
    }
    pub fn get_container(&self) -> Option<&Container>
    {
        self.container.as_ref()
    }
    pub fn get_source(&self) -> Option<&Source>
    {
        match self.get_header()
        {
            Some(h) => Some(h.get_source()),
            None => None
        }
    }
    pub fn container_zip_file_name(&self) -> Option<Cow<str>>
    {
        match self.communication.container.as_ref()
        {
            Some(c) => Some(Cow::from(&c.body)),
            None => None
        }
    }
    pub fn get_acknowledgment(&self) -> Option<&Acknowledgment>
    {
        self.communication.acknowledgment.as_ref()
    }
    pub fn get_document(&self) -> Option<&Document>
    {
        self.communication.document.as_ref()
    }
}


///Попытка распознать корневой xml файл
fn process_root_xml(file_path: &PathBuf, paths: &mut Vec<PathBuf>) -> Result<(Communication, Option<super::Ltr>, Option<Container>, bool), MedoParserError>
{
    let clear_xml = clear_xml(file_path, None)?;
    let de: Result<Communication, DeError> = quick_xml::de::from_reader(clear_xml.0.as_bytes());
    if de.is_err()
    {
        let err = de.err().as_ref().unwrap().to_string().replace("missing field", "отсуствует поле");
        return Err(MedoParserError::ParseError(format!("Ошибка десериализации: {} , {}", file_path.display(), err)));
        //TODO сейчас лень пробрасывать все ошибки, сделаю
    }

    if de.as_ref().unwrap().header.is_none()
    {
        //Xml данного транспортного пакета имеет неизвестную структуру
        //Если формат совсем неверный то будет ошибка, а это на случай если формат имеет заголовок
        //communication, но потом ничего нет
        return Err(MedoParserError::ParseError(format!("Файл {} имеет некорректную структуру, обязательный тэг header отсутсвует", file_path.display())));
    }

    let wrong_encoding = clear_xml.1;
    if wrong_encoding
    {
        logger::warn!("При обработке файла {} \r\nпроизошла ошибка определения кодировки", file_path.display());
    }
    else
    {
        logger::info!("Обработан файл {}", file_path.display());
    }
    let comm = de.unwrap();
    //парсим файл ltr
    let ltr = process_ltr(file_path);
    if comm.container.is_some()
    {
        let cont = process_container_xml(file_path, &comm.container.as_ref().unwrap().body, paths)?;
        Ok((comm, ltr.ok(), Some(cont), wrong_encoding))
    }
    else
    {
        Ok((comm, ltr.ok(), None, wrong_encoding))
    }
}

///Если не будет файла ltr то мы не сможем определить обратный адрес, и отправить уведомления, так что это ошибка
fn process_ltr(file_path: &PathBuf) -> Result<Ltr, MedoParserError>
{
    let mut base_dir = file_path.clone();
    base_dir.pop();
    if let Some(entries) = get_entries(&base_dir)
    {
        let mut filtered = entries
        .iter()
        .filter(|f|
            f.path()
            .extension()
            .is_some()
            && f.path()
                .extension()
                .as_ref()
                .unwrap() == &"ltr");

        if let Some(f) = filtered.next()
        {
            let ltr = Ltr::parse_file(&f.path())?;
            return Ok(ltr);
        }    
    }
    logger::warn!("В директории {} файл ltr не найден (в файле содержится адрес отправителя) уведомления по данному пакету не смогут быть доставлены", base_dir.display());
    return Err(MedoParserError::LtrError(["В директории ", &base_dir.display().to_string(), " файл не обнаружен"].concat())); 
}

///Если в рутовом xml присуствует ссылка на зиповский архив - контейнер, то его надо анзипнуть и распарсить
fn process_container_xml(file_path: &PathBuf, zip_file: &String, paths: &mut Vec<PathBuf>) -> Result<Container, MedoParserError>
{
    let mut dir = file_path.clone();
    //Получает абсолютный путь к директории
    dir.pop();
    //путь к архиву
    dir.push(zip_file);
    let xml_in_container = unzip(&dir, paths)?;
    let clear_xml = clear_xml(&xml_in_container, None)?;
    let de: Container = quick_xml::de::from_reader(clear_xml.0.as_bytes())?;
    //TODO попадается дубликат поля надо с этим что то делать, может уго удалять просто?
    logger::info!("Обработан файл {}", &xml_in_container.display());
    Ok(de)
}

///анзипим файлы контейнера попутно добавляя из в paths, и отдаем на обработку xml файл passport.xml (обычно у него такое наименование, но иногда может быть и другое, так что просто ищем в архиве первый попавшийся файл xml)
fn unzip(zip_file: &PathBuf, paths: &mut Vec<PathBuf>) -> Result<PathBuf, MedoParserError>
{
    let unzip_dir = "container";
    let zip_file = zip_file;
    let mut absolute_path_to_dir = zip_file.clone();
    absolute_path_to_dir.pop();
    absolute_path_to_dir.push(unzip_dir);
    let _ = std::fs::create_dir(&absolute_path_to_dir);
    let mut passport : Option<PathBuf> = None;
    let file = std::fs::File::open(zip_file.as_path());
    if let Ok(file) = file
    {
        let archive = zip::ZipArchive::new(file);
        if archive.is_err()
        {
            return Err(MedoParserError::UnzipError(format!("Ошибка открытия архива {} {}", zip_file.display(), archive.err().unwrap().to_string())));
        }
        let mut archive = archive.unwrap();
        for i in 0..archive.len() 
        {
            let mut file = archive.by_index(i).unwrap();
            let f_name = match file.enclosed_name() 
            {
                Some(path) => path.to_owned(),
                None => continue,
            };
            let file_path = Path::new(&absolute_path_to_dir)
            .join(&f_name);
            let mut outfile = std::fs::File::create(&file_path).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
            if extension_path_is(&file_path, "xml")
            {
                passport = Some(file_path);
            }
            let all_files = Path::new(unzip_dir)
            .join(&f_name);
            paths.push(all_files);
        }
        if passport.is_some()
        {
            return Ok(passport.unwrap());
        }
        else
        {
            return Err(MedoParserError::ZipEmpty(format!("В архиве {} не найдено ни одного файла xml", zip_file.display())));
        }
    }
    else
    {
        return Err(MedoParserError::UnzipError(format!("zip файл {} не существует в текущей директории", zip_file.display())));
    }
}

///Очистка xml от неймспейсов и попутно проверка на киррилицу, если нет киррилических знаков то значит ошибка анкодинга
///т.е. кодировка не utf-8 и не windows-1251
fn clear_xml(file_path: &PathBuf, enc: Option<FileEncoding>) -> Result<(String, bool), MedoParserError>
{
    let decoded = open_file(file_path.as_path(), enc)?;
    let clear_xml = cleary_xml_namespaces(&decoded.1);
    // while let Ok(we)= WRONG_ENCODING.lock().as_mut()
    // {
    //     **we = decoded.0;
    //     break;
    // }
    Ok((clear_xml, decoded.0))
}