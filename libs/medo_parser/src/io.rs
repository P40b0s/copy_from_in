use std::{path::{Path, PathBuf}, fs::{OpenOptions, DirEntry, File}, io::{BufWriter, Write, Read}};

use encoding::{all::WINDOWS_1251, DecoderTrap, Encoding};
use logger::{error, backtrace};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use serde_json::Value;

use crate::MedoParserError;

// lazy_static!(
//     static ref NS_REGEX: Regex = regex::Regex::new("xmlns:([a-z0-9A-Z]+)").unwrap();
//     static ref ENCODING_REGEX: Regex = regex::Regex::new("encoding=\"([a-z0-9A-Z-]+)\"").unwrap();
// );
pub static NS_REGEX: Lazy<Regex> = Lazy::new(|| 
{
    regex::Regex::new("xmlns:([a-z0-9A-Z]+)").unwrap()
});
pub static ENCODING_REGEX: Lazy<Regex> = Lazy::new(|| 
{
    regex::Regex::new("encoding=\"([a-z0-9A-Z-]+)\"").unwrap()
});

pub fn get_entries(path:&Path) -> Option<Vec<DirEntry>>
{
    let paths = std::fs::read_dir(path);
    if paths.is_err()
    {
        error!("😳 Ошибка чтения директории {} - {} {}", path.display(), paths.err().unwrap(), backtrace!());
        return None;
    }
    let mut dirs = vec![];
    for d in paths.unwrap()
    {
        let dir = d.unwrap();
        dirs.push(dir);
    }
    return Some(dirs);
}

pub fn extension_is(f: &DirEntry, ext:&str) -> bool
{
    f.path().extension().is_some() && f.path().extension().unwrap() == ext
}
pub fn extension_path_is(f: &PathBuf, ext:&str) -> bool
{
    f.extension().is_some() && f.extension().unwrap() == ext
}
///Получает все директории и файлы из указанной директории
pub fn get_files(path:&Path) -> Option<Vec<String>>
{
    let paths = std::fs::read_dir(path);
    if paths.is_err()
    {
        error!("😳 Ошибка чтения директории {} - {}", path.display(), paths.err().unwrap());
        return None;
    }
    let mut entry = vec![];
    for e in paths.unwrap()
    {
        if e.is_err()
        {
            error!("{}", e.err().unwrap());
            return None;
        }
        if let Some(dir) = e.unwrap().file_name().to_str()
        {
            entry.push(dir.to_owned());
        }
        else
        {
            error!("Невозможно получить имя файла в директории {}", path.display());
        }   
    }
    return Some(entry);
}

pub fn write_value_to_file(data: &Value, file_name: &str) -> bool
{
    let pretty = serde_json::to_string_pretty(&data);
    if pretty.is_err()
    {
        error!("Ошибка записи сериализованных данных в файл! {}", pretty.err().unwrap().to_string());
        return false;
    }
    let path = file_name.to_owned();
    let write = OpenOptions::new()
    .write(true)
    .create(true)
    .open(path);

    let mut f = BufWriter::new(write.unwrap());
    let write = f.write_all(pretty.unwrap().as_bytes());
    if write.is_err()
    {
        logger::error!("{}", write.err().unwrap());
        return false;
    }
    else 
    {
        return true;
    }
   
}


pub fn write_string_to_file(data: &str, file_name: &str) -> bool
{
    let path =  file_name.to_owned();
    let write = OpenOptions::new()
    .write(true)
    .create(true)
    .open(path);
    if write.is_err()
    {
        error!("Ошибка записи сериализованных данных в файл! {}", write.err().unwrap());
        return false;
    }
    let mut f = BufWriter::new(write.unwrap());
    let file = f.write_all(data.as_bytes());
    if file.is_err()
    {
        logger::error!("{}", file.err().unwrap());
        return false;
    }
    else 
    {
        return true;
    }
}

///Если не указано явно, сначала пробует открыть файл в utf-8 если возникнет ошибка то пробует перевести кодировку в windows-1251
/// и открыть, если в открытом файле не находит букву а... за это вот стыдно, но перебирать несколько киррилических символов неохота
/// да и слишком такое на удачу... то ставит метку что есть ошибка в определении кодировки
pub fn open_file(path: &Path, encoding: Option<FileEncoding>) -> Result<(bool, String), MedoParserError>
{
    let mut bytes = Vec::new();
    //let mut ok_encoding = true;
    let file = std::fs::File::open(path);
    if file.is_err()
    {
        return Err(MedoParserError::ParseError(format!("Ошибка открытия файла {}: {}", path.display(), file.as_ref().err().unwrap())));
    }
    let binary = file.as_ref().unwrap().read_to_end(&mut bytes);
    if binary.is_err()
    {
        return Err(MedoParserError::ParseError(format!("Ошибка чтения файла {}: {}", path.display(), binary.as_ref().err().unwrap())));
    }
    let _ = file.as_ref().unwrap().read_to_end(&mut bytes);
    //Если указан конкретный энкодинг то сюда
    if let Some(e) = encoding
    {
        match e
        {
            FileEncoding::Utf8 => return enc_utf_8(&bytes, path),
            FileEncoding::Windows1251 => return enc_win1251(&bytes, path),
        }
    }
    //если не указан то пробуем utf-8, если ошибка то пробуем windows-1251
    return enc_utf_8(&bytes, path);

    fn enc_utf_8(bytes: &[u8], path: &Path) -> Result<(bool, String), MedoParserError>
    {
        let utf8 = std::str::from_utf8(&bytes);
        if utf8.is_err()
        {
            return enc_win1251(bytes, path);
        }
        else 
        {
            let utf8 = utf8.unwrap();
            if !utf8.contains("а") || !utf8.contains("о") || !utf8.contains("е")
            {
                return Ok((true, utf8.to_owned()));
            }
            else
            {   
                return Ok((false,  utf8.to_owned()));
            }
        }
    }
    fn enc_win1251(bytes: &[u8], path: &Path) -> Result<(bool, String), MedoParserError>
    {
        let result = WINDOWS_1251.decode(&bytes, DecoderTrap::Strict);
        if result.is_err()
        {
            return Err(MedoParserError::ParseError(format!("Ошибка открытия файла в кодировке windows-1251 {}: \r\n{}", path.display(), result.as_ref().err().unwrap())));
        }
        let result = result.unwrap();
        //Если после преобразования найдены кириллические символы, то значит кодировка другая
        if !result.contains("а") || !result.contains("о") || !result.contains("е")
        {
            return Ok((true, result));
        }
        else
        {   
            return Ok((false, result));
        }
    }
}



///Читение файл в строку из чистого utf-8
pub fn read_file(file_path: &Path) -> Option<String>
{
    let file = std::fs::read_to_string(file_path);
    if file.is_err()
    {
        let err = file.err().unwrap();
       
        error!("Ошибка чтения файла {} - {}", file_path.display(), err);
        return None;
    }
    Some(file.unwrap())
}

///Чтение файла в бинарный формат
pub fn read_file_to_binary(file_path: &PathBuf) -> Option<Vec<u8>>
{
    if let  Ok(f) = File::open(file_path).as_mut()
    {
        //f.read(&mut buffer);
        let mut buffer = Vec::new();
        if f.read_to_end(&mut buffer).is_ok()
        {
            return Some(buffer);
        }
        else 
        {
            return None;
        }
    }
    None
}

///Сериализация объекта в строковый формат
#[warn(unused_assignments)]
pub fn serialize<T>(json : T, file_name : &str, directory: Option<&str>) where T : Clone + Serialize
{
    let mut work_dir = PathBuf::default();
    if directory.is_some()
    {
        let p = PathBuf::from(directory.unwrap());
        work_dir = p;
        work_dir.push(file_name);
    }
    else
    {
        work_dir = std::env::current_dir().unwrap();
        work_dir.push(file_name);
    }
    let write = OpenOptions::new()
    .write(true)
    .truncate(true)
    .create(true)
    //.create(true)
    .open(work_dir);
    if write.is_err()
    {
        error!("{}", write.err().unwrap());
        return;
    }
    let pretty = serde_json::to_string_pretty(&json);
    if let Ok(pretty) = pretty
    {
        let mut f = BufWriter::new(write.unwrap());
        let _write = f.write_all(pretty.as_bytes());
    }
    else
    {
        error!("Ошибка десериализации файла {} -> {}", write.err().unwrap(), pretty.err().unwrap());
        return;
    }
    // work_dir.push(file_name);
    // let path = std::path::Path::new(&work_dir);
    // let mut file = std::fs::File::create(&path).unwrap();
    // file.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes()).unwrap();
    // file.flush().unwrap();
}

///Очистка пространств имен для файла
pub fn cleary_xml_namespaces(xml: &str) -> String
{
    //logger::StructLogger::initialize_logger();
    let mut xml : String = xml.to_owned();
    let mut ns = String::new();
    for cap in NS_REGEX.captures_iter(&xml) 
    {
        let n =  [&cap[1], ":\n"].concat();
        ns.push_str(&n);
    }
    for l in ns.lines()
    {
        xml = xml.replace(l, "");
    }
    xml
}

//Получение кодировки декларируемой в xml
pub fn get_xml_ecoding(text: &str) -> FileEncoding
{
    let mut enc = String::new();
    for cap in ENCODING_REGEX.captures_iter(text) 
    {
        enc =  [&cap[1], ":\n"].concat();
    }
    match enc.as_str()
    {
        "utf-8" => FileEncoding::Utf8,
        "windows-1251" => FileEncoding::Windows1251,
        //"windows-1252" => FileEncoding::Windows1252,
        _ => FileEncoding::Utf8,
    }
}

///Копируем рекурсивно все файлы и директории
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> std::io::Result<()>
{
    std::fs::create_dir_all(&destination)?;
    for entry in std::fs::read_dir(source)? 
    {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() 
        {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } 
        else 
        {
            std::fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub enum FileEncoding
{
    Windows1251,
    //Windows1252,
    ///По умолчанию у всех utf-8
    Utf8
}

