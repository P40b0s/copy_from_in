use std::path::Path;

use medo_parser::Packet;
use settings::{Settings, Task};

use crate::Error;



pub struct Services{}

impl Services
{
    pub fn clear_dirs(settings: &Settings) -> Result<u32, Error>
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
                            let err = ["Ошибка очистки пакета ", d, " -> ", e.as_ref()].concat();
                            logger::error!("{}", &err);
                            errors.push(err);
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
        settings.clean_excludes();
        if errors.len() > 0
        {
            return Err(Error::ServiceErrors(errors));
        }
        {
            return Ok(count);
        }
    }

    // pub fn clear_excepts(tasks: &Vec<Task>) -> u32
    // {
    //     let mut count: u32 = 0;
    //     for t in tasks
    //     {
    //         let mut guard = EXCLUDES.get().unwrap().lock().unwrap();
    //         let excludes = guard.get(t.get_task_name()).unwrap().clone();
    //         let mut del: Vec<String> = vec![];
    //         if let Some(dirs) = super::io::get_dirs(t.get_source_dir()) 
    //         {
    //             for ex in &excludes
    //             {
    //                 if dirs.contains(ex)
    //                 {
    //                     del.push(ex.to_owned());
    //                 }
    //                 else
    //                 {
    //                     count+=1;
    //                 }
    //             }
    //         }
    //         guard.insert(t.get_task_name().to_owned(), del);
    //     }
    //     logger::info!("При проверке списка задач исключено {} несуществующих директорий", count);
    //     count
    // }
}

#[cfg(test)]
mod tests
{
    use settings::{FileMethods, Serializer, Settings};

    use crate::copyer::service::Services;

    #[test]
    fn test_dir_cleaner()
    {
        logger::StructLogger::initialize_logger();
        let s = Settings::load(true, Serializer::Toml).unwrap();
        let _ = Services::clear_dirs(&s);
        println!("{:?}", s);
    }
}