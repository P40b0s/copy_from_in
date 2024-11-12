use std::sync::Arc;
use db_service::{from_json, get_fields_for_update, query, to_json, CountRequest, DbError, FromRow, IdSelector, QuerySelector, Result, Row, Selector, SortingOrder, SqlOperations, SqlitePool, SqliteRow};
use transport::{PacketInfo, Packet};
use super::addresse_table::AddresseTable;

#[derive(Debug)]
pub struct PacketTable
{
    id: String,
    packet_info: PacketInfo,
    task_name: String,
    report_sended: bool
}
impl PacketTable
{
    pub fn new(packet: &Packet) -> Self
    {
        Self 
        { 
            id: packet.get_id().to_owned(),
            packet_info: packet.get_packet_info().to_owned(),
            task_name: packet.get_task().get_task_name().to_owned(),
            report_sended: packet.report_sended
        }
    }
    pub fn get_packet_info(&self) -> &PacketInfo
    {
        &self.packet_info
    }
    pub fn get_task_name(&self) -> &str
    {
        &self.task_name
    }
    pub fn report_is_sended(&self) -> bool
    {
        self.report_sended
    }
}

impl FromRow<'_, SqliteRow> for PacketTable
{
    fn from_row(row: &SqliteRow) -> Result<Self> 
    {
        let id: String =  row.try_get("id")?;
        let files = serde_json::from_str::<Vec<String>>(row.try_get("files")?).unwrap();
        Ok(
        Self
        {
            id,
            task_name: row.try_get("task_name")?,
            report_sended: row.try_get("report_sended")?,
            packet_info: PacketInfo
            {
                 //sender info нам нужен только при парсинге пакета, потом мы данные из него передаем в таблицу адресов
                sender_info: None,
                header_guid: row.try_get("header_id")?,
                packet_directory: row.try_get("directory")?,
                packet_type: row.try_get("packet_type")?,
                delivery_time: row.try_get("delivery_time")?,
                default_pdf: row.try_get("default_pdf")?,
                files,
                requisites: from_json(row, "requisites"),
                sender_id: row.try_get( "sender_id")?,
                wrong_encoding: false,
                error: row.try_get("error")?,
                pdf_hash: row.try_get("pdf_hash")?,
                acknowledgment: from_json(row, "acknowledgment"),
                trace_message: row.try_get("trace_message")?,
                update_key: "".to_string(),
                visible: row.try_get("visible")?,
            }
        })
    }
}

impl<'a> SqlOperations<'a> for PacketTable
{
    fn get_id(&'a self) -> &'a str
    {
        &self.id
    }
    fn table_name() -> &'static str 
    {
       "packets"
    }
    fn table_fields() -> &'a[&'static str]
    {
        &[
            "id", //0
            "task_name", //1
            "header_id", //2
            "directory", //3
            "packet_type", //4
            "delivery_time", //5
            "error", //6
            "default_pdf", //7
            "pdf_hash", //8
            "files", //9
            "requisites", //10
            "sender_id", //11
            "acknowledgment", //12
            "visible", //13
            "trace_message", //14
            "report_sended" //15
        ]
    }
    
    fn create_table() -> String 
    {  
        ["CREATE TABLE IF NOT EXISTS ", Self::table_name(), " (
            ", Self::table_fields()[0], " TEXT PRIMARY KEY NOT NULL,
            ", Self::table_fields()[1], " TEXT NOT NULL,
            ", Self::table_fields()[2], " TEXT, 
            ", Self::table_fields()[3], " TEXT NOT NULL, 
            ", Self::table_fields()[4], " TEXT,
            ", Self::table_fields()[5], " TEXT NOT NULL,
            ", Self::table_fields()[6], " TEXT,
            ", Self::table_fields()[7], " TEXT, 
            ", Self::table_fields()[8], " TEXT,
            ", Self::table_fields()[9], " JSON DEFAULT('[]'),
            ", Self::table_fields()[10], " JSON,
            ", Self::table_fields()[11], " TEXT,
            ", Self::table_fields()[12], " JSON,
            ", Self::table_fields()[13], " INTEGER NOT NULL DEFAULT 1,
            ", Self::table_fields()[14], " TEXT,
            ", Self::table_fields()[15], " INTEGER NOT NULL DEFAULT 0
            );"].concat()
    }
    
    async fn update(&'a self, pool: Arc<SqlitePool>) -> Result<u64, DbError>
    {
        let update_set = get_fields_for_update(Self::table_fields());
        let sql = ["UPDATE ", Self::table_name(),
        " SET ", &update_set ," WHERE ", Self::table_fields()[0]," = $1"].concat();

        let res = query(&sql)
        .bind(self.id.to_string())
        .bind(self.get_task_name())
        .bind(self.packet_info.header_guid.as_ref())
        .bind(&self.packet_info.packet_directory)
        .bind(self.packet_info.packet_type.as_ref())
        .bind(&self.packet_info.delivery_time)
        .bind(self.packet_info.error.as_ref())
        .bind(self.packet_info.default_pdf.as_ref())
        .bind(self.packet_info.pdf_hash.as_ref())
        .bind(to_json(&self.packet_info.files))
        .bind(to_json(&self.packet_info.requisites))
        .bind(self.packet_info.sender_id.as_ref())
        .bind(to_json(&self.packet_info.acknowledgment))
        .bind(&self.packet_info.visible)
        .bind(self.packet_info.trace_message.as_ref())
        .bind(self.report_is_sended())
        .execute(&*pool).await?;
        Ok(res.rows_affected())
    }
    async fn add_or_replace(&'a self, pool: Arc<SqlitePool>) -> Result<u64, DbError>
    {
        let sql = Self::insert_or_replace_query();
        let res = query(&sql)
        .bind(self.id.to_string())
        .bind(self.get_task_name())
        .bind(self.packet_info.header_guid.as_ref())
        .bind(&self.packet_info.packet_directory)
        .bind(self.packet_info.packet_type.as_ref())
        .bind(&self.packet_info.delivery_time)
        .bind(self.packet_info.error.as_ref())
        .bind(self.packet_info.default_pdf.as_ref())
        .bind(self.packet_info.pdf_hash.as_ref())
        .bind(to_json(&self.packet_info.files))
        .bind(to_json(&self.packet_info.requisites))
        .bind(self.packet_info.sender_id.as_ref())
        .bind(to_json(&self.packet_info.acknowledgment))
        .bind(&self.packet_info.visible)
        .bind(self.packet_info.trace_message.as_ref())
        .bind(self.report_is_sended())
        .execute(&*pool).await?;
        // if let Ok(addreesses) = AddresseTable::try_from(&self.packet_info)
        // {
        //     let _ = addreesses.add_or_ignore(Arc::clone(&pool)).await;
        // }
        Ok(res.rows_affected())
    }
    async fn add_or_ignore(&'a self, pool: Arc<SqlitePool>) -> Result<u64, DbError>
    {
        let sql = Self::insert_or_ignore_query();
        let res = query(&sql)
        .bind(self.id.to_string())
        .bind(self.get_task_name())
        .bind(self.packet_info.header_guid.as_ref())
        .bind(&self.packet_info.packet_directory)
        .bind(self.packet_info.packet_type.as_ref())
        .bind(&self.packet_info.delivery_time)
        .bind(self.packet_info.error.as_ref())
        .bind(self.packet_info.default_pdf.as_ref())
        .bind(self.packet_info.pdf_hash.as_ref())
        .bind(to_json(&self.packet_info.files))
        .bind(to_json(&self.packet_info.requisites))
        .bind(self.packet_info.sender_id.as_ref())
        .bind(to_json(&self.packet_info.acknowledgment))
        .bind(&self.packet_info.visible)
        .bind(self.packet_info.trace_message.as_ref())
        .bind(self.report_is_sended())
        .execute(&*pool).await?;
        // if let Ok(addreesses) = AddresseTable::try_from(&self.packet_info)
        // {
        //     let _ = addreesses.add_or_ignore(Arc::clone(&pool)).await;
        // }
        Ok(res.rows_affected())
    }
}

impl PacketTable
{
    pub async fn packets_count(pool: Arc<SqlitePool>, names: Vec<String>) -> Result<u32, DbError>
    {
        let vq = Self::only_visible_tasks_query(names);
        let selector = Selector::new_concat(&["SELECT COUNT(*) as count FROM ", Self::table_name()])
        .add_raw_query(&vq);
        let count: CountRequest = Self::get_one(&selector, pool).await?;
        Ok(count.count)
    }

    ///`rows` - количество записей получаемых из базы данных<br>
    /// `offset` - с какой позиции начинать
    pub async fn get_with_offset(rows: u32, offset: u32, pool: Arc<SqlitePool>, names: Vec<String>) -> Result<Vec<PacketTable>, DbError>
    {
        let vq = Self::only_visible_tasks_query(names);
        let ids_offset_selector = Selector::new_concat(&["SELECT id FROM ", Self::table_name()])
        .sort(SortingOrder::Desc("delivery_time"))
        .add_raw_query(&vq)
        .limit(&rows)
        .offset(&offset);
        let users_ids: Vec<IdSelector> = Self::select_special_type(&ids_offset_selector, Arc::clone(&pool)).await?;
        let id_in = users_ids.into_iter().map(|m| m.0).collect::<Vec<String>>();
        let selector = Selector::new(&Self::full_select())
        .where_in(&id_in)
        .sort(SortingOrder::Desc("delivery_time"));
        let packets = Self::select(&selector, pool).await?;
        Ok(packets)
    }
    pub async fn search(search_string: &str, pool: Arc<SqlitePool>) -> Result<Vec<PacketTable>, DbError>
    {
        let like = vec![
            Self::like("directory", search_string),
            Self::like("requisites", search_string),
        ];
        let select = [" where ".to_owned(), like.join(" OR ")].concat();
        let selector = Selector::new(&Self::full_select())
        .sort(SortingOrder::Desc("delivery_time"))
        .add_raw_query(&select)
        .limit(&100);
        let packets = Self::select(&selector, pool).await?;
        Ok(packets)
    }
    fn like(field_name: &str, searched_word: &str) -> String
    {
        [field_name, " LIKE ", "'%", searched_word, "%'"].concat()
    }

    fn only_visible_tasks_query(names: Vec<String>) -> String
    {
        let mut params: String = " where ".to_owned();
        for (i,t) in names.iter().enumerate()
        {
            let p = if i + 1 < names.len()
            {
                [" task_name = ", "\"", t, "\" or "].concat()
            }
            else
            {
                [" task_name = ", "\"", t, "\""].concat()
            };
            params.push_str(&p);
        }
        params
    }
    ///Удаление из БД по имени таска + имени директории
    pub async fn truncate(task_name: &str, dirs: &[String], pool: Arc<SqlitePool>)
    {
        for d in dirs
        {
            let sql = ["DELETE FROM ", &Self::table_name(), " WHERE task_name = $1", " AND directory = $2" ].concat();
            let _ = db_service::query(&sql)
            .bind(task_name)
            .bind(d)
            .execute(&*pool).await;
        }
    }
    pub async fn delete_by_task_name(task_name: &str, pool: Arc<SqlitePool>)
    {
        let sql = ["DELETE FROM ", &Self::table_name(), " WHERE task_name = $1"].concat();
        let _ = db_service::query(&sql)
        .bind(task_name)
        .execute(&*pool).await;
    }
    pub async fn delete_by_id(id: &str, pool: Arc<SqlitePool>)
    {
        let sql = ["DELETE FROM ", &Self::table_name(), " WHERE id = $1"].concat();
        let _ = db_service::query(&sql)
        .bind(id)
        .execute(&*pool).await;
    }

    pub async fn select_all(pool: Arc<SqlitePool>) -> Result<Vec<PacketTable>, DbError> 
    {
        let selector = Selector::new(PacketTable::full_select());
        PacketTable::select(&selector, pool).await
    }
}

#[cfg(test)]
mod tests
{
    use db_service::to_json;
    use transport::PacketInfo;

    #[test]
   fn test_json_null()
   {
    let s : Option<PacketInfo> = None;
    let json = to_json(&s);
    println!("{:?}", json);
   }

}



