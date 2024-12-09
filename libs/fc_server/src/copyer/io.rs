use std::fs::DirEntry;
use std::time::SystemTime;
use std::{path::{Path, PathBuf}, sync::Arc}; //time::SystemTime
use futures::FutureExt;
use logger::{backtrace, debug, error};
use tokio::task::JoinSet;
use crate::Error;

/// копирование директорий с задержкой для проверки полностью ли скопирован файл в эту директорию
/// * check_duration с такой переодичностью идет проверка изменилось ли время изменения файла или нет
pub fn copy_recursively_async(source: Arc<PathBuf>, destination: Arc<PathBuf>, check_duration: u64) -> futures::future::BoxFuture<'static, anyhow::Result<(), Error>>
{
    async move 
    {
        let read_dir = tokio::fs::read_dir(source.as_ref()).await;
        if read_dir.is_err()
        {
            return Err(Error::Io(read_dir.err().unwrap()));
        }
        let mut count = 0;
        let mut read_dir = read_dir.unwrap();
        while let Some(_) = read_dir.next_entry().await?
        {
            count+=1;
        }
        if count == 0
        {
            //если в начальной пусто то все правильно, если во вложенной то просто возвращаем 0 в предыдущую функцию
            //вызываться будет ниже
            return Ok(());
        }
        //let start = std::time::SystemTime::now();
        let create_dir = tokio::fs::create_dir_all(&destination.as_ref()).await;
        if create_dir.is_err()
        {
            return Err(Error::Io(create_dir.err().unwrap()));
        }
        let mut files: Vec<(PathBuf, PathBuf)> = vec![];
        let mut entry_count = 0;
        //let mut handles : Vec<JoinHandle<std::io::Result<(PathBuf, PathBuf)>>> = Vec::new();
        let mut set = JoinSet::new();
        let read_iter = tokio::fs::read_dir(source.as_ref()).await;
        if read_iter.is_err()
        {
            return Err(Error::Io(read_iter.err().unwrap()));
        }
        let dirs_iter = tokio::fs::read_dir(source.as_ref()).await;
        if dirs_iter.is_err()
        {
            return Err(Error::Io(dirs_iter.err().unwrap()));
        }
        let mut dirs_iter = dirs_iter.unwrap();
        while let Some(entry) = dirs_iter.next_entry().await? 
        {
            let filetype = entry.file_type().await;
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
        let new_count_check = tokio::fs::read_dir(source.as_ref()).await;
        if new_count_check.is_err()
        {
            return  Err(Error::Io(new_count_check.err().unwrap()));
        }
        let mut count = 0;
        let mut new_count_check = new_count_check.unwrap();
        while let Some(_) = new_count_check.next_entry().await? 
        {
            count+=1;
        }
        if entry_count != count
        {
            debug!("За время копирования файлов в исходной директории зафиксированно изменение количества файлов с `{}` на `{}`, процедура копирования перезапущена...", entry_count, count);
            let _ = copy_recursively_async(source, destination, check_duration).await?;
        }
        //копируем все файлы в списке
        for f in files
        {
            debug!("Копирование `{}` в `{}`...", f.0.display(), f.1.display());
            let iscopy = tokio::fs::copy(f.0, f.1).await;
            if iscopy.is_err()
            {
                return Err(Error::Io(iscopy.err().unwrap()));
            }
        }
        //let end = std::time::SystemTime::now();
        //let duration = end.duration_since(start).unwrap();
        //return Box::pin(async { Ok(duration.as_secs())});
        return Ok(());
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
        let metadata = tokio::fs::metadata(source_file_path.as_ref()).await?;
        let mtime = metadata.modified();
        if let Ok(md_time) = mtime
        {
            if modifed_time.is_none() &&  modifed_len.is_none()
            {
                modifed_time = Some(md_time);
                modifed_len = Some(metadata.len());
                //logger::debug!("file_len:{}->o:{}n:{}", source_file_path.as_ref().display(), modifed_len.as_ref().unwrap(), metadata.len());
                //logger::debug!("file_modifed_time:{}->o:{:?}n:{:?}", source_file_path.as_ref().display(), modifed_time.as_ref().unwrap_or(&SystemTime::UNIX_EPOCH), &md_time);
            }
            else
            {
                //logger::debug!("file_len:{}->o:{}n:{}", source_file_path.as_ref().display(), modifed_len.as_ref().unwrap(), metadata.len());
                //logger::debug!("file_modifed_time:{}->o:{:?}n:{:?}", source_file_path.as_ref().display(), modifed_time.as_ref().unwrap_or(&SystemTime::UNIX_EPOCH), &md_time);
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
//FIXME на 2008 сервере не работает апи используемое по умолчанию в windows-sys для получения времени модификации файла
// async fn check_file<P: AsRef<Path>>(source_file_path: P, dest_file_path: P, check_duration: u64) -> anyhow::Result<(P, P), Error>
// {
//     let mut modifed_time: Option<u32> = None;
//     let mut modifed_len: Option<u64> = None;
//     let mut max_repeats = 30;
//     loop
//     {
//         //в цикле сверяем время изменения файла и его длинну каждые N секунд, если время изменилось, ждем еще N секунд, иначе добавляем в список на копирование
//         let metadata = std::fs::metadata(source_file_path.as_ref())?;
//         let mtime = filetime::FileTime::from_last_modification_time(&metadata);
//         //if let Ok(md_time) = metadata.modified()
//         let md_time = mtime.nanoseconds();
//         {
//             if modifed_time.is_none() &&  modifed_len.is_none()
//             {
//                 modifed_time = Some(md_time);
//                 modifed_len = Some(metadata.len());
//                 //logger::debug!("file_len:{}->o:{}n:{}", source_file_path.as_ref().display(), modifed_len.as_ref().unwrap(), metadata.len());
//                 //logger::debug!("file_modifed_time:{}->o:{:?}n:{:?}", source_file_path.as_ref().display(), modifed_time.as_ref().unwrap_or(&SystemTime::UNIX_EPOCH), &md_time);
//             }
//             else
//             {
//                 //logger::debug!("file_len:{}->o:{}n:{}", source_file_path.as_ref().display(), modifed_len.as_ref().unwrap(), metadata.len());
//                 //logger::debug!("file_modifed_time:{}->o:{:?}n:{:?}", source_file_path.as_ref().display(), modifed_time.as_ref().unwrap_or(&SystemTime::UNIX_EPOCH), &md_time);
//                 if modifed_len.as_ref().unwrap() == &metadata.len() && modifed_time.as_ref().unwrap() ==  &md_time
//                 {
//                     modifed_time = None;
//                     modifed_len = None;
//                     return Ok((source_file_path, dest_file_path));
//                 }
//                 if modifed_time.as_ref().unwrap() !=  &md_time
//                 {
//                     modifed_time = Some(md_time);
//                 }
//                 if modifed_len.as_ref().unwrap() != &metadata.len()
//                 {
//                     modifed_len = Some(metadata.len())
//                 }
//             }
//         }
//         max_repeats -= 1;
//         if max_repeats == 0 
//         {
//             let err = format!("Превышено максимальное количество попыток при попытке копирования файла {:?}, файл должен успевать копироваться в систему в течении 2 минут", source_file_path.as_ref());
//             error!("{} {}", &err, backtrace!());
//             return Err(Error::FileTimeCopyError(err));
//         }
//         tokio::time::sleep(tokio::time::Duration::from_millis(check_duration)).await;
//     }
// }






// pub fn extension_is(f: &DirEntry, ext:&str) -> bool
// {
//     f.path().extension().is_some() && f.path().extension().unwrap() == ext
// }
// pub fn extension_path_is(f: &PathBuf, ext:&str) -> bool
// {
//     f.extension().is_some() && f.extension().unwrap() == ext
// }
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
#[cfg(test)]
mod tests
{
    #[test]
    fn test_copy()
    {
       
    }
}