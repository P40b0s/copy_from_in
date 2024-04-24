use std::path::Path;

use medo_parser::Packet;
use settings::{Settings, Task};

use crate::{copyer::io::get_files, Error};

use super::io::get_dirs;

pub trait PacketsCleaner
{
    ///нам нужно вернуть только колчество удаленных пакетов, ошибки нас не интересуют
    fn clear_packets(settings: &Settings) -> Result<u32, Error>
    {
        let mut count = 0;
        for t in &settings.tasks
        {
            if t.clean_types.len() > 0
            {   
                if let Some(dirs) = super::io::get_dirs(t.get_source_dir()) 
                {
                    for d in &dirs
                    {
                        let source_path = Path::new(t.get_source_dir()).join(d);
                        let packet = Packet::parse(&source_path);
                        if let Some(e) = packet.get_error()
                        {
                            let wrn = ["Директория ", d, " не является пакетом ", e.as_ref()].concat();
                            logger::warn!("{}", &wrn);
                            if let Some(files) = get_files(&source_path)
                            {
                                if files.is_empty()
                                {
                                    let _ = std::fs::remove_dir_all(&source_path);
                                    let inf = ["В задаче ", t.get_task_name(), " удалена пустая директория ", &source_path.display().to_string()].concat();
                                    logger::info!("{}", inf);
                                    count+=1;
                                }
                            }
                        }
                        else 
                        {
                            if let Some(pt) = packet.get_packet_type()
                            {
                                let pt = pt.into_owned();
                                //logger::info!("{} {:?}", &pt, t.clean_types);
                                if t.clean_types.contains(&pt)
                                {
                                    let _ = std::fs::remove_dir_all(&source_path);
                                    let inf = ["Пакет ", &source_path.display().to_string(), " типа `", &pt, "` в задаче " , t.get_task_name(),"  удален"].concat();
                                    logger::info!("{}", inf);
                                    count+=1;
                                }
                            }
                            // else 
                            // {
                            //     let err = ["Ошибка очистки пакета ", d, " тип пакета не найден в схеме xml не обнаружен"].concat();
                            //     logger::warn!("{}", &err);
                            //     errors.push(err);
                            // }
                        }
                    }
                }
            }
        }
        // if !errors.is_empty()
        // {
        //     return Err(Error::ServiceErrors(errors));
        // }
        {
            return Ok(count);
        }
    }
}

impl PacketsCleaner for Settings{}

pub trait ExcludesCreator
{
    ///Создание нового стоп листа имен директорий если лист уже есть, он будет пересканирован и сохранен заново
    async fn create_stoplist_file(task: &Task)
    {
        Settings::clear_exclude(task.get_task_name());
        if let Some(dirs) = get_dirs(&task.source_dir)
        {
            for d in dirs
            {
                Settings::add_to_exclude(task.get_task_name(), &d);
            }
        }
        Settings::save_exclude(task.get_task_name());
    }
}
impl ExcludesCreator for Task{}

#[cfg(test)]
mod tests
{
    use settings::{FileMethods, Serializer, Settings};

    use crate::copyer::service::PacketsCleaner;

    #[test]
    fn test_task_cleaner()
    {
        logger::StructLogger::initialize_logger();
        let s = Settings::load(Serializer::Toml).unwrap();
        let r = s.truncate_excludes();
        println!("{:?} => {}", s, r);
    }
    #[test]
    fn test_packets_cleaner()
    {
        logger::StructLogger::initialize_logger();
        let s = Settings::load(Serializer::Toml).unwrap();
        let r = Settings::clear_packets(&s);
        assert!(r.as_ref().unwrap() == &31);
        println!("{:?} => {}", s, r.unwrap());
    }
}