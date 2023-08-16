use std::{path::{Path, PathBuf}, fs::{OpenOptions, DirEntry, File}, io::{BufWriter, Write, Read}, borrow::{Cow, Borrow}, sync::Arc, time::Instant, fmt::Display};

use logger::{error, backtrace};
use serde::Serialize;
use serde_json::Value;
use encoding::{all::WINDOWS_1251, DecoderTrap, Encoding};
 
 /// Copy files from source to destination recursively.
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
            let dest = destination.as_ref().join(entry.file_name());
            std::fs::copy(entry.path(), &dest)?;
            if entry.path().extension().unwrap() == "zip"
            {
                unzip(&dest);
            }
        }
    }
    Ok(())
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
pub fn get_files<P: AsRef<Path>>(path: P) -> Option<Vec<DirEntry>>
{
    let paths = std::fs::read_dir(path.as_ref());
    if paths.is_err()
    {
        error!("😳 Ошибка чтения директории {} -> {}", path.as_ref().display(), paths.err().unwrap());
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
        if let Some(_) = e.as_ref().unwrap().file_name().to_str()
        {
            entry.push(e.unwrap());
        }
        else
        {
            error!("Невозможно получить имя файла в директории {}", path.as_ref().display());
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

///Читение файл в строку из чистого utf-8
// pub fn read_file(file_path: &Path) -> Option<String>
// {
//     let file = std::fs::read_to_string(file_path);
//     if file.is_err()
//     {
//         let err = file.err().unwrap();
       
//         error!("Ошибка чтения файла {} - {}", file_path.display(), err);
//         return None;
//     }
//     Some(file.unwrap())
// }

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

//Проверить существует ли указанная директория
pub fn path_is_exists<P: AsRef<Path>>(path: P ) -> bool
{
    let target_path = Path::new(path.as_ref());
    if let Ok(e) = target_path.try_exists()
    {
        if !e
        {
            //let err = ["Директория ", path.as_ref(), " не существует!"].concat();
            //error!("{}", err);
            return false;
        }
        else 
        {
            return true;
        }
    }
    else 
    {
        return false;
    }
}

///Если не указано явно, сначала пробует открыть файл в utf-8 если возникнет ошибка то пробует перевести кодировку в windows-1251
/// и открыть, если в открытом файле не находит букву а... за это вот стыдно, но перебирать несколько киррилических символов неохота
/// да и слишком такое на удачу... то ставит метку что есть ошибка в определении кодировки
pub fn read_file(path: &Path) -> Option<String>
{
    let mut bytes = Vec::new();
    //let mut ok_encoding = true;
    let file = std::fs::File::open(path);
    if file.is_err()
    {
        logger::error!("Ошибка открытия файла {}: {}", path.display(), file.as_ref().err().unwrap());
        return None;
    }
    let binary = file.as_ref().unwrap().read_to_end(&mut bytes);
    if binary.is_err()
    {
        logger::error!("Ошибка чтения файла {}: {}", path.display(), binary.as_ref().err().unwrap());
        return None;
    }
    let _ = file.as_ref().unwrap().read_to_end(&mut bytes);
    //если не указан то пробуем utf-8, если ошибка то пробуем windows-1251
    return enc_utf_8(&bytes, path);

    fn enc_utf_8(bytes: &[u8], path: &Path) -> Option<String>
    {
        let utf8 = std::str::from_utf8(&bytes);
        if let Ok(u8) = utf8
        {
            return Some(u8.to_owned());
        }
        else 
        {
            return enc_win1251(bytes, path);
        }
    }
    fn enc_win1251(bytes: &[u8], path: &Path) -> Option<String>
    {
        let result = WINDOWS_1251.decode(&bytes, DecoderTrap::Strict);
        if result.is_err()
        {
            logger::error!("Ошибка открытия файла в кодировке windows-1251 {}: \r\n{}", path.display(), result.as_ref().err().unwrap());
            return None;
        }
        return result.ok(); 
    }
}

///анзипим файлы контейнера попутно добавляя из в paths, и отдаем на обработку xml файл passport.xml (обычно у него такое наименование, но иногда может быть и другое, так что просто ищем в архиве первый попавшийся файл xml)
fn unzip(zip_file: &PathBuf)
{
    let unzip_dir = "container";
    let zip_file = zip_file;
    let mut absolute_path_to_dir = zip_file.clone();
    absolute_path_to_dir.pop();
    absolute_path_to_dir.push(unzip_dir);
    let _ = std::fs::create_dir(&absolute_path_to_dir);
    let file = std::fs::File::open(zip_file.as_path());
    if let Ok(file) = file
    {
        let archive = zip::ZipArchive::new(file);
        if archive.is_err()
        {
            logger::error!("Ошибка открытия архива {} {}", zip_file.display(), archive.err().unwrap().to_string());
            return
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
        }
    }
    else
    {
        logger::error!("Файл {} не существует в текущей директории", zip_file.display());
        return
    }
    logger::info!("Успешно распакован архив транспортного пакета {}", zip_file.display());
}