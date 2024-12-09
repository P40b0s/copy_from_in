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

