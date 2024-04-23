use std::{fmt::Display, fs::{DirEntry, File, OpenOptions}, future::Future, io::{BufWriter, Read, Write}, ops::Deref, os::unix::fs::MetadataExt, path::{Path, PathBuf}, pin::Pin, sync::Arc, time::SystemTime};

use futures::FutureExt;
use logger::{backtrace, debug, error};
use serde_json::Value;
use tokio::{task::{JoinHandle, JoinSet}, try_join};

use crate::Error;

 
 /// копирование директорий с задержкой для проверки полностью ли скопирован файл в эту директорию
 /// * check_duration с такой переодичностью идет проверка изменилось ли время изменения файла или нет
 pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>, check_duration: u64) -> std::io::Result<u64> 
 {
    if std::fs::read_dir(source.as_ref())?.count() == 0
    {
        return Ok(0);
    }
    let start = std::time::SystemTime::now();
    std::fs::create_dir_all(&destination)?;
    let mut files: Vec<(PathBuf, PathBuf)> = vec![];
    let mut entry_count = 0;
    //debug!("Копирование `{}` в `{}`...", source.as_ref().display(), destination.as_ref().display());
    for entry in std::fs::read_dir(source.as_ref())? 
    {
        let entry = entry?;
        let filetype = entry.file_type()?;
        entry_count += 1;
        if filetype.is_dir() 
        {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()), check_duration)?;
        }
        else 
        {
            let dest = destination.as_ref().join(entry.file_name());
            let mut modifed_time: Option<SystemTime> = None;
            loop
            {
                //в цикле сверяем время изменения файла каждые N секунд, если время изменилось, ждем еще N секунд, иначе добавляем в список на копирование
                let metadata = std::fs::metadata(&entry.path())?;
                if let Ok(md_time) = metadata.modified()
                {
                    if modifed_time.is_none()
                    {
                        modifed_time = Some(md_time);
                    }
                    else
                    {
                        if modifed_time.as_ref().unwrap() <  &md_time
                        {
                            modifed_time = Some(md_time);
                        }
                        else
                        {
                            modifed_time = None;
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
    let count = new_count_check.count();
    //debug!("новое колчиество файлов `{}` старое количество файлов `{}`...", entry_count, count);
    if entry_count != count
    {
        return copy_recursively(source, destination, check_duration);
    }
    //копируем все файлы в списке
    for f in files
    {
        debug!("Копирование `{}` в `{}`...", f.0.display(), f.1.display());
        std::fs::copy(f.0, f.1)?;
    }
    let end = std::time::SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    Ok(duration.as_secs())
 }

 /// копирование директорий с задержкой для проверки полностью ли скопирован файл в эту директорию
 /// * check_duration с такой переодичностью идет проверка изменилось ли время изменения файла или нет
 pub fn copy_recursively_async(source: Arc<PathBuf>, destination: Arc<PathBuf>, check_duration: u64) -> futures::future::BoxFuture<'static, anyhow::Result<u64, Error>>
 {
    async move 
    {
        let read_dir = std::fs::read_dir(source.as_ref());
        if read_dir.is_err()
        {
            return Err(Error::Io(read_dir.err().unwrap()));
        }
        if read_dir.unwrap().count() == 0
        {
            //если в начальной пусто то все правильно, если во вложенной то просто возвращаем 0 в предыдущую функцию
            //вызываться будет ниже
            return Ok(0);
        }
        let start = std::time::SystemTime::now();
        let create_dir = std::fs::create_dir_all(&destination.as_ref());
        if create_dir.is_err()
        {
            return Err(Error::Io(create_dir.err().unwrap()));
        }
        let mut files: Vec<(PathBuf, PathBuf)> = vec![];
        let mut entry_count = 0;
        //let mut handles : Vec<JoinHandle<std::io::Result<(PathBuf, PathBuf)>>> = Vec::new();
        let mut set = JoinSet::new();
        let read_iter = std::fs::read_dir(source.as_ref());
        if read_iter.is_err()
        {
            return Err(Error::Io(read_iter.err().unwrap()));
        }

        let dirs_iter = std::fs::read_dir(source.as_ref());
        if dirs_iter.is_err()
        {
            return Err(Error::Io(dirs_iter.err().unwrap()));
        }
        for entry in dirs_iter.unwrap() 
        {
            let entry = entry;
            if entry.is_err()
            {
                return Err(Error::Io(entry.err().unwrap()));
            }
            let entry = entry.unwrap();
            let filetype = entry.file_type();
            if filetype.is_err()
            {
                return Err(Error::Io(filetype.err().unwrap()));
            }
            let filetype = filetype.unwrap();
            let path = entry.path();
            entry_count +=1;
            if filetype.is_dir() 
            {
                let new_dest = Path::new(destination.as_ref()).join(entry.file_name());
                let _ = copy_recursively_async(Arc::new(entry.path()), Arc::new(new_dest), check_duration).await?;
            }
            else 
            {
                let dest = destination.as_ref().join(entry.file_name());
                set.spawn(check_file(path, dest, check_duration));
            }
        }
        while let Some(res) = set.join_next().await 
        {
            let out = res.map_err(|e| Error::FileTimeCopyError(e.to_string()));
            if let Ok(result) = out
            {
                if result.is_err()
                {
                    return Err(Error::FileTimeCopyError(result.err().unwrap().to_string()));
                }
                files.push(result.unwrap());
            }
            else 
            {
                return Err(out.err().unwrap());
            }
            //let ready = out?;
        }
        //после проверки всех файлов проверяем не появились ли в директории новые файлы, если появились запускаем процедуру сначала
        let new_count_check = std::fs::read_dir(source.as_ref());
        if new_count_check.is_err()
        {
            return  Err(Error::Io(new_count_check.err().unwrap()));
        }
        let count = new_count_check.unwrap().count();
        if entry_count != count
        {
            debug!("За время копирования файлов в исходной директории зафиксированно изменение количества файлов с `{}` на `{}`, процедура копирования перезапущена...", entry_count, count);
            let _ = copy_recursively_async(source, destination, check_duration).await?;
        }
        //копируем все файлы в списке
        for f in files
        {
            debug!("Копирование `{}` в `{}`...", f.0.display(), f.1.display());
            let iscopy = std::fs::copy(f.0, f.1);
            if iscopy.is_err()
            {
                return Err(Error::Io(iscopy.err().unwrap()));
            }
        }
        let end = std::time::SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        //return Box::pin(async { Ok(duration.as_secs())});
        return Ok(duration.as_secs());
    }.boxed()
 }


async fn check_file<P: AsRef<Path>>(source_file_path: P, dest_file_path: P, check_duration: u64) -> anyhow::Result<(P, P), Error>
{
    let mut modifed_time: Option<SystemTime> = None;
    let mut modifed_len: Option<u64> = None;
    let mut max_repeats = 30;
    loop
    {
        //в цикле сверяем время изменения файла и его длинну каждые N секунд, если время изменилось, ждем еще N секунд, иначе добавляем в список на копирование
        let metadata = std::fs::metadata(source_file_path.as_ref())?;
        if let Ok(md_time) = metadata.modified()
        {
            if modifed_time.is_none() &&  modifed_len.is_none()
            {
                modifed_time = Some(md_time);
                modifed_len = Some(metadata.len());
                logger::debug!("file_len:{}->o:{}n:{}", source_file_path.as_ref().display(), modifed_len.as_ref().unwrap(), metadata.len());
                logger::debug!("file_modifed_time:{}->o:{:?}n:{:?}", source_file_path.as_ref().display(), modifed_time.as_ref().unwrap_or(&SystemTime::UNIX_EPOCH), &md_time);
            }
            else
            {
                logger::debug!("file_len:{}->o:{}n:{}", source_file_path.as_ref().display(), modifed_len.as_ref().unwrap(), metadata.len());
                logger::debug!("file_modifed_time:{}->o:{:?}n:{:?}", source_file_path.as_ref().display(), modifed_time.as_ref().unwrap_or(&SystemTime::UNIX_EPOCH), &md_time);
                if modifed_len.as_ref().unwrap() == &metadata.len() && modifed_time.as_ref().unwrap() ==  &md_time
                {
                    modifed_time = None;
                    modifed_len = None;
                    return Ok((source_file_path, dest_file_path));
                }
                if modifed_time.as_ref().unwrap() !=  &md_time
                {
                    modifed_time = Some(md_time);
                }
                if modifed_len.as_ref().unwrap() != &metadata.len()
                {
                    modifed_len = Some(metadata.len())
                }
            }
        }
        max_repeats -= 1;
        if max_repeats == 0 
        {
            let err = format!("Превышено максимальное количество попыток при попытке копирования файла {:?}, файл должен успевать копироваться в систему в течении 2 минут", source_file_path.as_ref());
            error!("{} {}", &err, backtrace!());
            return Err(Error::FileTimeCopyError(err));
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(check_duration)).await;
    }
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
        logger::StructLogger::initialize_logger();
        super::copy_recursively(
        "/hard/xar/projects/rust/copy_from_in/test_data/in/38773995_1_1_unzip",
        "/hard/xar/projects/rust/copy_from_in/test_data/in/test_copy",
        1000);
    }
}