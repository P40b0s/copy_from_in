use std::{future::Future, ops::Deref, path::{Path, PathBuf}, pin::Pin, sync::{Arc, Mutex, RwLock}};
use db_service::{SqlOperations, SqlitePool};
use hashbrown::{HashMap, HashSet};
use logger::{debug, error};
use once_cell::sync::{Lazy, OnceCell};
use settings::{Settings, Task};
use utilites::io::{get_dirs, get_dirs_async};
use futures::future::{BoxFuture, FutureExt};

use crate::{db::ExceptTable, Error};

pub struct ExcludesService<T>(pub T) where T: ExcludesTrait;
impl<T> Deref for ExcludesService<T> where T: ExcludesTrait
{
    type Target = T;
    fn deref(&self) -> &Self::Target 
    {
        &self.0
    }
}

///Чтобы сделать object safe trait нужно навешать гору лапши еще сверху...
pub trait ExcludesTrait
{
    ///Добавить к задаче имя директории, чтобы больше ее не копировать
    ///если возвращает true то директория успешно добавлена в список, если false то такая директория там уже есть
    fn add<'a>(&'a self, task_name: &str, dir: &str) -> BoxFuture<'a, Result<bool, Error>>;
    ///Удалить директорию из определенного таска, например если нужно заново пересканировать этот пакет
    fn delete<'a>(&'a self, task_name: &str, dir: &str) -> BoxFuture<'a, Result<(), Error>>;
    //Обрезать файл с исключениями (*.task) удаляет из файла все директории которые отсутсвуют в текущий момент 
    // по пути source_dir в текущей задаче
    //fn truncate<'a>(&'a self, tasks: &[Task]) -> BoxFuture<'a, Result<u64, Error>>;
    ///Удаление таблицы
    fn clear<'a>(&'a self, task_name: &str) -> BoxFuture<'a, Result<(), Error>>;
    ///Заменяет текущую таблицу если она есть, или создает новую
    fn replace<'a>(&'a self, task: &Task) -> BoxFuture<'a, Result<(), Error>>;
}

pub struct FileExcludes
{
    dictionary: OnceCell<Mutex<hashbrown::hash_map::HashMap<String, Vec<String>>>>
}
impl FileExcludes
{
    ///Загрузка всех исключений из файлов
    pub fn new(tasks: &[Task]) -> Self
    {
        let slf = Self { dictionary: OnceCell::new() };
        let dictionary = slf.dictionary.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = dictionary.lock().unwrap();
        for t in tasks
        {
            if !guard.contains_key(t.name.as_str())
            {
                let file = [&t.name, ".task"].concat();
                let path = Path::new(&file);
                let ex: Result<Vec<String>, utilites::error::Error> = utilites::deserialize(&path, true, utilites::Serializer::Json);
                if let Ok(mut res) = ex
                {
                    res.sort();
                    guard.insert(t.name.clone(), res);
                };
            }
        }
        drop(guard);
        slf  
    }
    ///сохранение исключений в файл
    fn save(&self, task_name: &str)
    {
        let concat_path = [task_name, ".task"].concat();
        let file_name = Path::new(&concat_path);
        let guard = self.dictionary.get().unwrap().lock().unwrap();
        if let Some(vec) = guard.get(task_name)
        {
            if let Err(e) = utilites::serialize(vec, file_name, true, utilites::Serializer::Json)
            {
                logger::error!("Ошибка сохранения исключений списка {} -> {}", &concat_path, e);
            }
        }  
    }
}

// impl ExcludesTrait for FileExcludes
// {
//     fn add(&self, task_name: &str, dir: &str) -> Result<bool, Error>
//     {
        
//         let value = self.dictionary.get_or_init(|| Mutex::new(HashMap::new()));
//         let mut guard = value.lock().unwrap();
//         if !guard.contains_key(task_name)
//         {
//             guard.insert(task_name.to_owned(), vec![dir.to_owned()]);
//             self.save(task_name);
//             return Ok(true);
//         }
//         else 
//         {
//             if let Some(ex) = guard.get_mut(task_name)
//             {
//                 let d = dir.to_owned();
//                 if !ex.contains(&d)
//                 {
//                     ex.push(dir.to_owned());
//                     self.save(task_name);
//                     return Ok(true);
//                 }
//                 else 
//                 {
//                     return Ok(false);
//                 }
//             }
//         }
//         return Ok(false);
//     }

//     fn delete(&self, task_name: &str, dir: &str) -> Result<(), Error>
//     {
       
//         let mut guard = self.dictionary.get().unwrap().lock().unwrap();
//         if let Some(v) = guard.get_mut(task_name)
//         {
//             v.retain(|r| r != dir);
//         }
//         self.save(task_name);
//         Ok(())
//     }

//     fn truncate(&self, tasks: &[Task]) -> Result<u64, Error>
//     {
//         let mut count: u32 = 0;
//         let mut task_count: u32 = 0;
//         for t in tasks
//         {
//             let mut guard = self.dictionary.get().unwrap().lock().unwrap();
//             let excludes = guard.get_mut(t.get_task_name()).unwrap();
//             task_count = excludes.len() as u32;
//             if let Some(dirs) = get_dirs(t.get_source_dir()) 
//             {
//                 if task_count >= dirs.len() as u32
//                 {
//                     count += task_count - dirs.len() as u32;
//                 }
//                 *excludes = dirs;
//                 self.save(t.get_task_name());
//             }
//             drop(guard);
//             if count > 0
//             {
//                 logger::info!("При проверке списка задачи {} исключено {} несуществующих директорий",  t.get_task_name(), count);
//             }
//         }
//         Ok(count as u64)
//     }

//     fn clear(&self, task_name: &str) -> Result<(), Error>
//     {
//         let excl = self.dictionary.get_or_init(|| Mutex::new(HashMap::new()));
//         let mut guard = excl.lock().unwrap();
//         if guard.contains_key(task_name)
//         {
//             guard.remove(task_name);
//         }
//         self.save(task_name);
//         Ok(())
//     }

//     fn replace(&self, task: &Task) -> Result<(), Error> 
//     {
//         self.clear(task.get_task_name())?;
//         if let Some(dirs) = get_dirs(&task.source_dir)
//         {
//             let excl = self.dictionary.get_or_init(|| Mutex::new(HashMap::new()));
//             let mut guard = excl.lock().unwrap();
//             let excludes = guard.get_mut(task.get_task_name()).unwrap();
//             *excludes = dirs;
//             self.save(task.get_task_name());
//         }
//         Ok(())
//     }
// }


pub struct SqliteExcludes
{
    db_pool: Arc<SqlitePool>,
    cache: tokio::sync::RwLock<HashSet<String>>
}
impl SqliteExcludes
{
    pub async fn new(pool: Arc<SqlitePool>) -> Self
    {
        let h = ExceptTable::get_hashes(Arc::clone(&pool)).await.unwrap_or_default();
        let hs = HashSet::from_iter(h);
        Self 
        {
            db_pool: pool,
            cache: tokio::sync::RwLock::new(hs)
        }
    }

    pub fn get_pool(&self) -> Arc<SqlitePool>
    {
        Arc::clone(&self.db_pool)
    }
}

impl ExcludesTrait for SqliteExcludes
{
    fn add<'a>(&'a self, task_name: &str, dir: &str) -> BoxFuture<'a, Result<bool, Error>>
    {
        let pool = self.get_pool();
        let hash = utilites::Hasher::hash_from_strings([task_name, dir]);
        let task_name = task_name.to_owned();
        let dir = dir.to_owned();
        Box::pin(async move
        {
            let contains_guard = self.cache.read().await;
            let contains = contains_guard.contains(&hash);
            drop(contains_guard);
            if !contains
            {
                let added = ExceptTable::new(&task_name, &dir).add_or_ignore(pool).await?;
                let mut insert_guard = self.cache.write().await;
                insert_guard.insert(hash);
                Ok(added > 0)
            }
            else 
            {
                Ok(false)
            }
        })
    }

    fn delete<'a>(&'a self, task_name: &str, dir: &str) -> BoxFuture<'a, Result<(), Error>>
    {
        let pool = self.get_pool();
        let hash = utilites::Hasher::hash_from_strings([task_name, dir]);
        let task_name = task_name.to_owned();
        let dir = dir.to_owned();
        Box::pin(async move
        {
            let res = ExceptTable::delete(&task_name, &dir, pool).await?;
            if res > 0
            {
                let mut guard = self.cache.write().await;
                guard.remove(&hash);
            }
            Ok(())
        })
    }

    // fn truncate<'a>(&'a self, tasks: &[Task]) -> BoxFuture<'a, Result<u64, Error>>
    // {
    //     let mut count = 0;
    //     let pool = self.get_pool();
    //     let tasks = tasks.to_owned();
    //     Box::pin(async move
    //     { 
    //         let mut guard = self.cache.write().await;
    //         *guard = HashSet::new();
    //         drop(guard);
    //         for t in tasks
    //         {
    //             if let Ok(dirs) = utilites::io::get_dirs_async(t.get_source_dir()).await 
    //             {
    //                 let dirs_count = dirs.len();
    //                 let task_count =   ExceptTable::truncate(dirs, t.get_task_name(), Arc::clone(&pool)).await? as usize;
    //                 if task_count >= dirs_count
    //                 {
    //                     count = task_count - dirs_count;
    //                 }
    //             }
    //             if count > 0
    //             {
    //                 logger::info!("При проверке списка задачи {} исключено {} несуществующих директорий",  t.get_task_name(), count);
    //             }
    //         }
    //         Ok(count as u64)
    //     })
    // }

    fn clear<'a>(&'a self, task_name: &str) -> BoxFuture<'a, Result<(), Error>>
    {
        let pool = self.get_pool();
        let task_name = task_name.to_owned();
        Box::pin(async move
        { 
            let all_excludes = ExceptTable::get_hashes_by_task_name(&task_name, Arc::clone(&pool)).await?;
            let mut guard = self.cache.write().await;
            for e in &all_excludes
            {
                guard.remove(e);
            }
            drop(guard);
            let excl = ExceptTable::delete_task(&task_name, pool).await?;
            logger::info!("Удалено {} директорий из задачи {}", excl,  &task_name);
            Ok(())
        })
    }
    fn replace<'a>(&'a self, task: &Task) -> BoxFuture<'a, Result<(), Error>>
    {
        let pool = self.get_pool();
        let task = task.to_owned();
        Box::pin(async move
        { 
            self.clear(task.get_task_name()).await?;
            if let Ok(dirs) = get_dirs_async(&task.source_dir).await
            {
                ExceptTable::replace(dirs, task.get_task_name(), pool).await?;
            }
            Ok(())
        })
    }
}


#[cfg(test)]
mod db_tests
{
    use logger::debug;
    use super::ExcludesTrait;
   //добавление двух таблиц и их последующее их удаление
//     #[test]
//     fn test_redb_create_clear()
//     {
//         //let b : Box<dyn ExcludesTrait> = Box::new(super::KeyValueStore::new());
//         let _ = logger::StructLogger::new_default();
//         let t = super::ExcludesService(super::KeyValueStore::new());
//         let tasks = vec!["task_1", "task_2"];
//         let add_1 = t.add(tasks[0], "1");
//         let add_2 = t.add(tasks[1], "1");
//         debug!("{:?} {:?}", &add_1, &add_2);
//         assert!(add_1.is_ok() && add_2.is_ok());
//         let d = t.clear(tasks[0]);
//         assert!(d.is_ok());
//         let d2 = t.clear(tasks[1]);
//         assert!(d2.is_ok());
//     }
// /// добавление и удаление значения в таблицу
//     #[test]
//     fn test_redb_add_del()
//     {
//         let t = super::ExcludesService(super::KeyValueStore::new());
//         let tasks = vec!["task_1", "task_2"];
//         let add_1 = t.add(tasks[0], "1");
//         assert!(add_1.unwrap());
//         let del_1 = t.delete(tasks[0], "1");
//         assert!(del_1.is_ok()); 
//     }
//     ///попытка добавить дубликат значения в таблицу
//     #[test]
//     fn test_redb_skip_add_double()
//     {
//          let t = super::ExcludesService(super::KeyValueStore::new());
//         let tasks = vec!["task_1", "task_2"];
//         let add_1 = t.add(tasks[0], "21");
//         let add_2 = t.add(tasks[0], "21");
//         assert!(add_1.unwrap(), "Ошибка добавления значения");
//         assert!(!add_2.unwrap(), "Значение добавлено, хотя это дубликат и добавлено быть не должно!");
//         let del_1 = t.delete(tasks[0], "21");
//         assert!(del_1.is_ok(), "Удаление прошло не успешно"); 
//     }
}

// #[cfg(test)]
// mod files_tests
// {
//     use std::ops::Deref;
//     use logger::debug;
//     use super::{ExcludesService, ExcludesTrait};
//    ///добавление двух таблиц и их последующее их удаление
//     #[test]
//     fn test_redb_create_clear()
//     {
//         let b : Box<dyn ExcludesTrait> = Box::new(super::KeyValueStore::new());
//         let _ = logger::StructLogger::new_default();
//         let t = super::ExcludesService(super::KeyValueStore::new());
//         let tasks = vec!["task_1", "task_2"];
//         let add_1 = t.add(tasks[0], "1");
//         let add_2 = t.add(tasks[1], "1");
//         debug!("{:?} {:?}", &add_1, &add_2);
//         assert!(add_1.is_ok() && add_2.is_ok());
//         let d = t.clear(tasks[0]);
//         assert!(d.is_ok());
//         let d2 = t.clear(tasks[1]);
//         assert!(d2.is_ok());
//     }
// /// добавление и удаление значения в таблицу
//     #[test]
//     fn test_redb_add_del()
//     {
//         let t = super::ExcludesService(super::KeyValueStore::new());
//         let tasks = vec!["task_1", "task_2"];
//         let add_1 = t.add(tasks[0], "1");
//         assert!(add_1.unwrap());
//         let del_1 = t.delete(tasks[0], "1");
//         assert!(del_1.is_ok()); 
//     }
//     ///попытка добавить дубликат значения в таблицу
//     #[test]
//     fn test_redb_skip_add_double()
//     {
//          let t = super::ExcludesService(super::KeyValueStore::new());
//         let tasks = vec!["task_1", "task_2"];
//         let add_1 = t.add(tasks[0], "21");
//         let add_2 = t.add(tasks[0], "21");
//         assert!(add_1.unwrap(), "Ошибка добавления значения");
//         assert!(!add_2.unwrap(), "Значение добавлено, хотя это дубликат и добавлено быть не должно!");
//         let del_1 = t.delete(tasks[0], "21");
//         assert!(del_1.is_ok(), "Удаление прошло не успешно"); 
//     }
// }

#[cfg(test)]
mod sql_tests
{
    use std::{ops::Deref, sync::Arc};
    use logger::debug;
    use super::{ExcludesService, ExcludesTrait};
   ///добавление двух таблиц и их последующее их удаление
    #[tokio::test]
    async fn test_redb_create_clear()
    {
        let pool = Arc::new(db_service::new_connection("medo").await.unwrap());
        let b : Box<dyn ExcludesTrait> = Box::new(super::SqliteExcludes::new(pool).await);
        let _ = logger::StructLogger::new_default();
        //let t = super::ExcludesService(super::SqliteExcludes::new(pool));
        
        let tasks = vec!["task_1", "task_2"];
        // let add_1 = t.add(tasks[0], "1");
        // let add_2 = t.add(tasks[1], "1");
        // debug!("{:?} {:?}", &add_1, &add_2);
        // assert!(add_1.is_ok() && add_2.is_ok());
        // let d = t.clear(tasks[0]);
        // assert!(d.is_ok());
        // let d2 = t.clear(tasks[1]);
        // assert!(d2.is_ok());
    }
// добавление и удаление значения в таблицу
    // #[test]
    // fn test_redb_add_del()
    // {
    //     let t = super::ExcludesService(super::KeyValueStore::new());
    //     let tasks = vec!["task_1", "task_2"];
    //     let add_1 = t.add(tasks[0], "1");
    //     assert!(add_1.unwrap());
    //     let del_1 = t.delete(tasks[0], "1");
    //     assert!(del_1.is_ok()); 
    // }
    //попытка добавить дубликат значения в таблицу
    //#[test]
    // fn test_redb_skip_add_double()
    // {
    //      let t = super::ExcludesService(super::KeyValueStore::new());
    //     let tasks = vec!["task_1", "task_2"];
    //     let add_1 = t.add(tasks[0], "21");
    //     let add_2 = t.add(tasks[0], "21");
    //     assert!(add_1.unwrap(), "Ошибка добавления значения");
    //     assert!(!add_2.unwrap(), "Значение добавлено, хотя это дубликат и добавлено быть не должно!");
    //     let del_1 = t.delete(tasks[0], "21");
    //     assert!(del_1.is_ok(), "Удаление прошло не успешно"); 
    // }
}

