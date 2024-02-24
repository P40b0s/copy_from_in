use std::{fs::{DirEntry, File, OpenOptions}, io::{BufWriter, Read, Write}, path::{Path, PathBuf}, time::SystemTime};

use logger::error;
use serde_json::Value;

 
 /// копирование директорий с задержкой для проверки полностью ли скопирован файл в эту директорию
 pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>, check_duration: u64) -> std::io::Result<u64> 
 {
    let start = std::time::SystemTime::now();
    std::fs::create_dir_all(&destination)?;
    let mut files: Vec<(PathBuf, PathBuf)> = vec![];
    let mut entry_count = 0;
    for entry in std::fs::read_dir(source.as_ref())? 
    {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() 
        {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()), check_duration)?;
        }
        else 
        {
            entry_count += 1;
            let dest = destination.as_ref().join(entry.file_name());
            let mut size: Option<SystemTime> = None;
            loop
            {
                let metadata = std::fs::metadata(&entry.path())?;
                if let Ok(md_size) = metadata.modified()
                {
                    if size.is_none()
                    {
                        size = Some(md_size);
                    }
                    else
                    {
                        if size.as_ref().unwrap() <  &md_size
                        {
                            size = Some(md_size);
                        }
                        else
                        {
                            size = None;
                            files.push((entry.path(), dest.clone()));
                            break;
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(check_duration));
            }
        }
    }
    //после проверки всех файлов проверяем не появились ли в директории новые файлы, если появились запускаем процедуру сначала
    let new_count_check = std::fs::read_dir(source.as_ref())?;
    if entry_count != new_count_check.count()
    {
        return copy_recursively(source, destination, check_duration);
    }
    for f in files
    {
        std::fs::copy(f.0, f.1)?;
    }
    let end = std::time::SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    Ok(duration.as_secs())
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
    for file in paths.unwrap()
    {
        if file.is_err()
        {
            error!("😳{}", file.err().unwrap());
            return None;
        }
        if let Some(_) = file.as_ref().unwrap().file_name().to_str()
        {
            entry.push(file.unwrap());
        }
        else
        {
            error!("😳Невозможно получить имя файла в директории {}", path.as_ref().display());
        }   
    }
    return Some(entry);
}
pub fn get_dirs(path: &PathBuf) -> Option<Vec<String>>
{
    let paths = std::fs::read_dir(path);
    if paths.is_err()
    {
        error!("😳 Ошибка чтения директории {} - {}", path.display(), paths.err().unwrap());
        return None;
    }
    let mut dirs = vec![];
    for d in paths.unwrap()
    {
        let dir = d.unwrap().file_name().to_str().unwrap().to_owned();
        dirs.push(dir);
    }
    return Some(dirs);
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
#[cfg(test)]
mod tests
{
    #[test]
    fn test_copy()
    {
        super::copy_recursively("/home/phobos/projects/rust/copy_from_in/test_data/in/38773995_1_1_unzip (копия)", "/home/phobos/projects/rust/copy_from_in/test_data/in/test_copy", 10000);
    }
}