use std::path::Path;

use medo_parser::Packet;
use settings::{Settings, Task};

use crate::{copyer::io::get_files, Error};

pub trait PacketsCleaner
{
    fn clear_packets(settings: &Settings) -> Result<u32, Error>
    {
        let mut errors: Vec<String> = vec![];
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
                                logger::info!("{} {:?}", &pt, t.clean_types);
                                if t.clean_types.contains(&pt)
                                {
                                    let _ = std::fs::remove_dir_all(&source_path);
                                    let inf = ["Пакет ", &source_path.display().to_string(), " типа `", &pt, "` в задаче " , t.get_task_name(),"  удален"].concat();
                                    logger::info!("{}", inf);
                                    count+=1;
                                }
                            }
                            else 
                            {
                                let err = ["Ошибка очистки пакета ", d, " тип пакета не найден"].concat();
                                logger::error!("{}", &err);
                                errors.push(err);
                            }
                        }
                    }
                }
            }
        }
        settings.truncate_excludes();
        if !errors.is_empty()
        {
            return Err(Error::ServiceErrors(errors));
        }
        {
            return Ok(count);
        }
    }
}

impl PacketsCleaner for Settings{}

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