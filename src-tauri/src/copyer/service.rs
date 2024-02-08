use std::path::Path;

use medo_parser::Packet;
use settings::Task;

use crate::Error;

use super::directories_spy::EXCLUDES;

pub struct Services{}

impl Services
{
    pub fn clear_dirs(tasks: &Vec<Task>) -> Result<(), Error>
    {
        let mut errors: Vec<String> = vec![];
        for t in tasks
        {
            if t.cleaning
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
                            if let Some(pt) = packet.get_document_type()
                            {

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
        Self::clear_excepts(tasks);
        if errors.len() > 0
        {
            return Err(Error::ServiceErrors(errors));
        }
        {
            return Ok(());
        }
    }

    pub fn clear_excepts(tasks: &Vec<Task>) -> u32
    {
        let mut count: u32 = 0;
        for t in tasks
        {
            let mut guard = EXCLUDES.get().unwrap().lock().unwrap();
            let excludes = guard.get(t.get_task_name()).unwrap();
            if let Some(dirs) = super::io::get_dirs(t.get_source_dir()) 
            {
                for ex in excludes
                {
                    if !dirs.contains(ex)
                    {
                        excludes.retain(|r| r != ex);
                        count+=1;
                    }
                }
            }
        }
        logger::info!("При проверке списка задач исключено {} несуществующих директорий", count);
        count
    }
}

