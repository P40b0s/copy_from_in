use std::{borrow::Cow, ops::Deref, sync::Arc};
use db_service::{from_json, get_fields_for_update, query, to_json, CountRequest, DbError, FromRow, IdSelector, Operations, QuerySelector, Result, Row, Selector, SortingOrder, SqlOperations, SqlitePool, SqliteRow};
use logger::backtrace;
use transport::{Ack, PacketInfo, Requisites, SenderInfo, Packet};
use serde_json::json;
use settings::Task;
use uuid::Uuid;
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
                header_guid: row.try_get("header_id")?,
                packet_directory: row.try_get("directory")?,
                packet_type: row.try_get("packet_type")?,
                delivery_time: row.try_get("delivery_time")?,
                default_pdf: row.try_get("default_pdf")?,
                files,
                requisites: from_json(row, "requisites"),
                sender_info: from_json(row, "sender_info"),
                wrong_encoding: false,
                error: row.try_get("error")?,
                pdf_hash: row.try_get("pdf_hash")?,
                acknowledgment: from_json(row, "acknowledgment"),
                trace_message: row.try_get("trace_message")?,
                update_key: row.try_get("update_key")?,
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
            "sender_info", //11
            "acknowledgment", //12
            "update_key", //13
            "visible", //14
            "trace_message", //15
            "report_sended" //16
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
            ", Self::table_fields()[11], " JSON,
            ", Self::table_fields()[12], " JSON,
            ", Self::table_fields()[13], " TEXT NOT NULL,
            ", Self::table_fields()[14], " INTEGER NOT NULL DEFAULT 1,
            ", Self::table_fields()[15], " TEXT,
            ", Self::table_fields()[16], " INTEGER NOT NULL DEFAULT 0
            );"].concat()
    }
    
    async fn update(&'a self, pool: Arc<SqlitePool>) -> Result<(), DbError>
    {
        let update_set = get_fields_for_update(Self::table_fields());
        let sql = ["UPDATE ", Self::table_name(),
        " SET ", &update_set ," WHERE ", Self::table_fields()[0]," = $1"].concat();

        query(&sql)
        .bind(self.id.to_string())
        .bind(self.get_task_name())
        .bind(self.packet_info.header_guid.as_ref())
        .bind(&self.packet_info.delivery_time)
        .bind(self.packet_info.error.as_ref())
        .bind(self.packet_info.default_pdf.as_ref())
        .bind(self.packet_info.pdf_hash.as_ref())
        .bind(to_json(&self.packet_info.files))
        .bind(to_json(&self.packet_info.requisites))
        .bind(to_json(&self.packet_info.sender_info))
        .bind(to_json(&self.packet_info.acknowledgment))
        .bind(&self.packet_info.update_key)
        .bind(&self.packet_info.visible)
        .bind(self.packet_info.trace_message.as_ref())
        .bind(self.report_is_sended())
        .execute(&*pool).await?;
        if let Ok(addreesses) = AddresseTable::try_from(&self.packet_info)
        {
            let _ = addreesses.add_or_replace(Arc::clone(&pool)).await;
        }
        Ok(())
    }
    async fn add_or_replace(&'a self, pool: Arc<SqlitePool>) -> Result<(), DbError>
    {
        let sql = Self::insert_or_replace_query();
        query(&sql)
        .bind(self.id.to_string())
        .bind(self.get_task_name())
        .bind(self.packet_info.header_guid.as_ref())
        .bind(&self.packet_info.delivery_time)
        .bind(self.packet_info.error.as_ref())
        .bind(self.packet_info.default_pdf.as_ref())
        .bind(self.packet_info.pdf_hash.as_ref())
        .bind(to_json(&self.packet_info.files))
        .bind(to_json(&self.packet_info.requisites))
        .bind(to_json(&self.packet_info.sender_info))
        .bind(to_json(&self.packet_info.acknowledgment))
        .bind(&self.packet_info.update_key)
        .bind(&self.packet_info.visible)
        .bind(self.packet_info.trace_message.as_ref())
        .bind(self.report_is_sended())
        .execute(&*pool).await?;
        Ok(())
    }
    async fn add_or_ignore(&'a self, pool: Arc<SqlitePool>) -> Result<(), DbError>
    {
        let sql = Self::insert_or_ignore_query();
        query(&sql)
        .bind(self.id.to_string())
        .bind(self.get_task_name())
        .bind(self.packet_info.header_guid.as_ref())
        .bind(&self.packet_info.delivery_time)
        .bind(self.packet_info.error.as_ref())
        .bind(self.packet_info.default_pdf.as_ref())
        .bind(self.packet_info.pdf_hash.as_ref())
        .bind(to_json(&self.packet_info.files))
        .bind(to_json(&self.packet_info.requisites))
        .bind(to_json(&self.packet_info.sender_info))
        .bind(to_json(&self.packet_info.acknowledgment))
        .bind(&self.packet_info.update_key)
        .bind(&self.packet_info.visible)
        .bind(self.packet_info.trace_message.as_ref())
        .bind(self.report_is_sended())
        .execute(&*pool).await?;
        Ok(())
    }
}

impl PacketTable
{
    pub async fn packets_count(pool: Arc<SqlitePool>) -> Result<u32, DbError>
    {
        //let q = ["SELECT COUNT(*) as count FROM ", Self::table_name()].concat();
        let selector = Selector::new_concat(&["SELECT COUNT(*) as count FROM ", Self::table_name()]);
        let count: CountRequest = Self::get_one(&selector, pool).await?;
        Ok(count.count)
    }
    //TODO добавить выборку по параметрам а не тупо всех подряд, будет и отсеивание по имени и еще по чему то
    ///`rows` - количество записей получаемых из базы данных<br>
    /// `offset` - с какой позиции начинать
    pub async fn get_with_offset(rows: u32, offset: u32, pool: Arc<SqlitePool>, params: Option<Vec<(&str, &str)>>) -> Result<Vec<PacketTable>, DbError>
    {
        let ids_offset_selector = Selector::new_concat(&["SELECT id FROM ", Self::table_name()])
        .add_params(params)
        .sort(SortingOrder::Asc("delivery_time"))
        .limit(&rows)
        .offset(&offset);
        let users_ids: Vec<IdSelector> = Self::select_special_type(&ids_offset_selector, Arc::clone(&pool)).await?;
        let id_in = users_ids.into_iter().map(|m| m.0).collect::<Vec<String>>();
        let selector = Selector::new(&Self::full_select())
        .where_in(&id_in)
        .sort(SortingOrder::Asc("delivery_time"));
        let packets = Self::select(&selector, pool).await?;
        Ok(packets)
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
    use super::PacketTable;


    // use super::{Operations, Selector, QuerySelector};
    // #[tokio::test]
    // async fn test_add_user()
    // {
    //     super::initialize().await;
    //     let id = "d428fc2b-db42-4737-a211-414ffc41809d".to_string();
    //     let dict_str = "fa77873a-92f7-42d1-9a19-a79e862b3fc1".to_owned();
    //     let user = User
    //     {
    //         id: id.clone(),
    //         name1: "Тест_2".into(),
    //         name2: "Тестович_2".into(),
    //         surname: "Тестов_2".into(),
    //         san_ticket_number: "123321123".into(),
    //         bornsday: "24.05.1983".into(),
    //         post: Dictionary{id: dict_str.clone(), name: "123".into()},
    //         department: Dictionary{id: dict_str.clone(), name: "123".into()},
    //         rank: Dictionary{id: dict_str.clone(), name: "123".into()},
    //         live_place: "Тестовое место жительства".into(),
    //         phones: vec![
    //             Phones{ phone_type: "тестовый".into(), phone_number: "32123".into(), is_main: false }
    //         ],
    //         tests: vec![
    //             DiseaseTest{ is_active: true, date: Date::new(2024, 1, 1).unwrap().val }
    //         ],
    //         diseases: vec![],
    //         statuses: vec![]
    //     };
    //     let _  = super::UsersTable::create().await;
    //     let _ = super::UsersTable::add_or_replace(&user).await;
    //     let selector_1 = Selector::new(&super::UsersTable::full_select())
    //     .add_param("u.id", &id);
    //     println!("{}", selector_1.query().0);
    //     let select = super::UsersTable::select(&selector_1).await.unwrap();
    //     println!("{:?}", &select);
    //     assert!(select.len() == 1);
    //     //let _ = super::DiseasesTable::delete(&d).await;
    //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
    // }
    // #[tokio::test]
    // async fn test_add_user()
    // {
    //     logger::StructLogger::initialize_logger();
    //     let paging : Vec<String> = PacketTable::get_with_offset(3, 0, None).await.unwrap().into_iter().map(|m| m.packet_info.delivery_time).collect();
    //     logger::debug!("{:?}", paging);
    // }

    // #[tokio::test]
    // async fn test_json_select()
    // {
    //     super::initialize().await;
    //     let selector_1 = Selector::new(&super::UsersTable::full_select())
    //     .add_json_param("phones->'is_main'", &false);
    //     println!("{}", selector_1.query().0);
    //     let select = super::UsersTable::select(&selector_1).await.unwrap();
    //     println!("{:?}", &select);
    //     assert!(select.len() == 1);
    //     //let _ = super::DiseasesTable::delete(&d).await;
    //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
    // }

    // #[tokio::test]
    // async fn test_diseases_user_select()
    // {
    //     logger::StructLogger::initialize_logger();
    //     let _ = super::initialize().await;
    //     let select = UsersTable::get_current_diseases_users().await.unwrap();
    //     assert!(select.len() == 1);
    //     //let _ = super::DiseasesTable::delete(&d).await;
    //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
    // }
    // #[tokio::test]
    // async fn test_vacations_user_select()
    // {
    //     let _ = super::initialize().await;
    //     let select = UsersTable::get_users_status().await.unwrap();
    //     assert!(select.len() == 3);
    //     //let _ = super::DiseasesTable::delete(&d).await;
    //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
    // }

}



