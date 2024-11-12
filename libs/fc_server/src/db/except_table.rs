use std::sync::Arc;

use db_service::{from_json, get_fields_for_update, query,  to_json, CountRequest, DbError, FromRow, QuerySelector, Result, Row, Selector, SqlOperations, SqlitePool, SqliteRow};
use transport::{PacketInfo, Senders};
use serde::{Deserialize, Serialize};

use transport::{ContactInfo, ContactType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExceptTable
{
    pub dir_name: String,
    pub task_id: String,
    
}

impl FromRow<'_, SqliteRow> for ExceptTable
{
    fn from_row(row: &SqliteRow) -> Result<Self> 
    {
        Ok(Self 
        {
            dir_name: row.try_get("dir_name")?,
            task_id: row.try_get("task_id")?,
           
        })
    }
}

impl<'a> SqlOperations<'a> for ExceptTable
{
    fn get_id(&'a self) -> &'a str
    {
        &self.dir_name
    }
    fn table_name() -> &'static str 
    {
       "exceptions"
    }
    fn table_fields() -> &'a[&'static str]
    {
        &[
            "dir_name", //0
            "task_id" //1
            
        ]
    }
    fn create_table() -> String 
    {  
        ["CREATE TABLE IF NOT EXISTS ", Self::table_name(), " (
            ", Self::table_fields()[0], " TEXT NOT NULL, "
            , Self::table_fields()[1], " TEXT NOT NULL, ",
            "PRIMARY KEY (", Self::table_fields()[0], ",", Self::table_fields()[1], ")",
            ");"].concat()
    }

    async fn update(&'a self, pool: Arc<SqlitePool>) -> Result<u64, DbError>
    {
        let update_set = get_fields_for_update(Self::table_fields());
        let sql = ["UPDATE ", Self::table_name(),
        " SET ", &update_set ," WHERE ", Self::table_fields()[0]," = $1"].concat();
        let res = query(&sql)
        .bind(self.dir_name.to_string())
        .bind(self.task_id.to_string())
        .execute(&*pool).await?;
        Ok(res.rows_affected())
    }

    async fn add_or_replace(&'a self, pool: Arc<SqlitePool>) -> Result<u64, DbError>
    {
        let sql = Self::insert_or_replace_query();
        let res = query(&sql)
        .bind(self.dir_name.to_string())
        .bind(self.task_id.to_string())
        .execute(&*pool).await?;
        Ok(res.rows_affected())
    }
    async fn add_or_ignore(&'a self, pool: Arc<SqlitePool>) -> Result<u64, DbError>
    {
        let sql = Self::insert_or_ignore_query();
        let res = query(&sql)
        .bind(self.dir_name.to_string())
        .bind(self.task_id.to_string())
        .execute(&*pool).await?;
        Ok(res.rows_affected())
    }
    
}

impl ExceptTable
{
    pub fn new(task_name: &str, dir_name: &str) -> Self
    {
        Self
        {
            dir_name: dir_name.to_owned(),
            task_id: task_name.to_owned()
        }
    }
    pub async fn exists(task_name: &str, dir_name: &str, pool: Arc<SqlitePool>) -> Result<bool, DbError>
    {
        let q = ["SELECT COUNT(*) as count FROM ", Self::table_name()].concat();
        let select = [" where dir_name=", "'", dir_name, "'", " AND ", " task_id=", "'", task_name, "'"].concat();
        let selector = Selector::new(&q).add_raw_query(&select);
        let count: CountRequest = Self::get_one(&selector, pool).await?;
        Ok(count.count > 0)
    }

    ///Возвращает количество удаленных директорий
    pub async fn truncate(dirs: Vec<String>, task_id: &str, pool: Arc<SqlitePool>) -> Result<u64, DbError>
    {
        let del_count = Self::delete_task(task_id, Arc::clone(&pool)).await?;
        let mut tx = pool.begin().await?;
        let vals: Vec<String> = dirs.into_iter().map(|v| ["(", &v, ",", task_id, ")"].concat()).collect();
        let q = ["INSERT INTO", Self::table_name()," (dir_name, task_id) VALUES " , &vals.join(",")].concat();
        let selector = Selector::new(&q);
        db_service::query(&selector.query().0)
        .execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(del_count)
    }
    pub async fn delete_task(task_id: &str, pool: Arc<SqlitePool>) -> Result<u64, DbError>
    {
        let q = ["DELETE FROM", Self::table_name()," WHERE task_id=('", task_id, "')"].concat();
        let selector = Selector::new(&q);
        let exe = Self::execute(&selector, pool).await?;
        Ok(exe)
    }

    
    // fn id<S: AsRef<str>>(task_name: S, packet_dir: S) -> String
    // {
    //     utilites::Hasher::hash_from_strings(&[task_name, packet_dir])
    // }
}
