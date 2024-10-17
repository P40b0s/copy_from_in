use std::{ops::Deref, path::PathBuf, sync::Mutex};

use logger::error;
use once_cell::sync::Lazy;
use settings::{Settings, Task};
use redb::{Database, Error, ReadableTable, ReadableTableMetadata, TableDefinition};

static DB_NAME: &'static str = "excludes.redb";

///Сервис для обработки пакетов-исключений с помощью redb storage
pub struct KeyValueStore
{
    db: Lazy<Mutex<Database>>
}
impl KeyValueStore
{
    pub fn new() -> Self
    {
        Self { db: Lazy::new(|| Mutex::new(Database::create(DB_NAME).unwrap())) }
    }
}
struct ExcludesService<T>(T) where T: ExcludesTrait;
impl<T> Deref for ExcludesService<T> where T: ExcludesTrait
{
    type Target = T;
    fn deref(&self) -> &Self::Target 
    {
        &self.0
    }
}


pub trait ExcludesTrait
{
    ///Добавить к задаче имя директории, чтобы больше ее не копировать
    ///если возвращает true то директория успешно добавлена в список, если false то такая директория там уже есть
    fn add(&self, task_name: &str, dir: &str) -> Result<bool, Error>;
    ///Удалить директорию из определенного таска, например если нужно заново пересканировать этот пакет
    fn delete(&self, task_name: &str, dir: &str) -> Result<(), Error>;
    ///Обрезать файл с исключениями (*.task) удаляет из файла все директории которые отсутсвуют в текущий момент 
    /// по пути source_dir в текущей задаче
    fn truncate(&self, tasks: &[Task]) -> Result<u64, Error>;
    ///Удаление таблицы
    fn clear(&self, task_name: &str) -> Result<(), Error>;
}


static DB : Lazy<Mutex<Database>> = Lazy::new(|| Mutex::new(Database::create(DB_NAME).unwrap()));
impl ExcludesTrait for KeyValueStore
{
    fn add(&self, task_name: &str, dir: &str) -> Result<bool, Error>
    {
        let mut added = false;
        let db = DB.lock().unwrap();
        let write_tr = db.begin_write()?;
        drop(db);
        {
            let td = TableDefinition::<&str, u8>::new(task_name);
            let mut table = write_tr.open_table(td)?;
            let get = table.get(dir)?;
            if get.is_none()
            {
                drop(get);
                let _ = table.insert(dir, 0)?;
                added = true;
            }
        }
        write_tr.commit()?;
        return Ok(added);
    }

    fn delete(&self, task_name: &str, dir: &str) -> Result<(), Error>
    {
        let db = DB.lock().unwrap();
        let write_tr = db.begin_write()?;
        drop(db);
        {
            let td = TableDefinition::<&str, u8>::new(task_name);
            let mut table = write_tr.open_table(td)?;
            let _ = table.remove(dir)?;
        }
        write_tr.commit()?;
        Ok(())
    }

    fn truncate(&self, tasks: &[Task]) -> Result<u64, Error>
    {
        let mut count: u64 = 0;
        let mut table_count: u64 = 0;
        for t in tasks
        {
            if let Some(dirs) = get_dirs(t.get_source_dir())
            {
                let db = DB.lock().unwrap();
                let write_tr = db.begin_write()?;
                drop(db);
                {
                    let td = TableDefinition::<&str, u8>::new(t.get_task_name());
                    table_count = write_tr.open_table(td.clone())?.len()?;
                    let _ = write_tr.delete_table(td.clone())?;
                    let mut table = write_tr.open_table(td)?;
                    for d in &dirs
                    {
                        let _ = table.insert(d.as_str(), 0)?;
                    }
                }
                write_tr.commit()?;
                //прибавляем к общему количеству разницу между количеством имеющихся директорий и количества записей в БД
                if table_count >= dirs.len() as u64
                {
                    count += table_count - dirs.len() as u64;
                }
            }
        }
        Ok(count)
    }

    fn clear(&self, task_name: &str) -> Result<(), Error>
    {
        let db = DB.lock().unwrap();
        let write_tr = db.begin_write()?;
        drop(db);
        {
            let td = TableDefinition::<&str, u8>::new(task_name);
            let _ = write_tr.delete_table(td)?;
        }
        write_tr.commit()?;
        Ok(())
    }
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
    use std::ops::Deref;
    use logger::debug;

    use crate::services::redb::ExcludesTrait;
    use super::ExcludesService;
   ///добавление двух таблиц и их последующее их удаление
    #[test]
    fn test_redb_create_clear()
    {
        logger::StructLogger::new_default();
        let t = super::ExcludesService(super::KeyValueStore::new());
        let tasks = vec!["task_1", "task_2"];
        let add_1 = t.add(tasks[0], "1");
        let add_2 = t.add(tasks[1], "1");
        debug!("{:?} {:?}", &add_1, &add_2);
        assert!(add_1.is_ok() && add_2.is_ok());
        let d = t.clear(tasks[0]);
        assert!(d.is_ok());
        let d2 = t.clear(tasks[1]);
        assert!(d2.is_ok());
    }
/// добавление и удаление значения в таблицу
    #[test]
    fn test_redb_add_del()
    {
        let t = super::ExcludesService(super::KeyValueStore::new());
        let tasks = vec!["task_1", "task_2"];
        let add_1 = t.add(tasks[0], "1");
        assert!(add_1.unwrap());
        let del_1 = t.delete(tasks[0], "1");
        assert!(del_1.is_ok()); 
    }
    ///попытка добавить дубликат значения в таблицу
    #[test]
    fn test_redb_skip_add_double()
    {
         let t = super::ExcludesService(super::KeyValueStore::new());
        let tasks = vec!["task_1", "task_2"];
        let add_1 = t.add(tasks[0], "21");
        let add_2 = t.add(tasks[0], "21");
        assert!(add_1.unwrap(), "Ошибка добавления значения");
        assert!(!add_2.unwrap(), "Значение добавлено, хотя это дубликат и добавлено быть не должно!");
        let del_1 = t.delete(tasks[0], "21");
        assert!(del_1.is_ok(), "Удаление прошло не успешно"); 
    }
}