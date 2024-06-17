use std::{borrow::Cow, ops::Deref};
use db_service::{from_json, get_connection, query, to_json, CountRequest, FromRow, Id, IdSelector, Operations, QuerySelector, Result, Row, Selector, SortingOrder, SqliteRow};
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



impl<'a> Id<'a> for PacketTable
{
    fn get_id(&'a self)-> Uuid
    {
        Uuid::parse_str(&self.id).unwrap()
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

impl<'a> Operations<'a> for PacketTable
{
    fn table_name() -> &'static str 
    {
       "medo"
    }
    fn base_name() -> &'static str 
    {
        "packets"
    }
    fn create_table() -> String 
    {  
        ["CREATE TABLE IF NOT EXISTS ", Self::table_name(), " (
            id TEXT PRIMARY KEY NOT NULL,
            task_name TEXT NOT NULL,
            header_id TEXT, 
            directory TEXT NOT NULL, 
            packet_type TEXT,
            delivery_time TEXT NOT NULL,
            error TEXT,
            default_pdf TEXT, 
            pdf_hash TEXT,
            files JSON DEFAULT('[]'),
            requisites JSON,
            sender_info JSON,
            acknowledgment JSON,
            update_key TEXT NOT NULL,
            visible INTEGER NOT NULL DEFAULT 1,
            trace_message TEXT,
            report_sended INTEGER NOT NULL DEFAULT 0,
            );"].concat()
    }
    fn full_select() -> String 
    {
        //SELECT header_id, directory, packet_type, delivery_time, error, default_pdf, 
        //files, requisites, sender_info, pdf_hash, update_key,
        // acknowledgment, visible, trace_message FROM packets";
        ["SELECT 
        id,
        task_name,
        header_id, 
        directory, 
        packet_type,
        delivery_time,
        error,
        default_pdf,
        files,
        requisites,
        sender_info,
        pdf_hash,
        update_key,
        acknowledgment,
        visible,
        trace_message,
        report_sended 
        FROM ", Self::table_name()].concat()
    }
    async fn update(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection(Self::base_name()).await?;
        let sql = ["UPDATE ", Self::table_name(),
        " SET 
        task_name = $2
        header_id = $3
        directory = $4,
        packet_type = $5,
        delivery_time = $6,
        error = $7,
        default_pdf = $8,
        files = $9,
        requisites = $10,
        sender_info = $11,
        pdf_hash = $12,
        update_key = $13,
        acknowledgment = $14,
        visible = $15,
        trace_message = $16,
        report_sended = $17
        WHERE id = $1"].concat();
        query(&sql)
        .bind(self.id.to_string())
        .bind(&self.task_name)
        .bind(&self.packet_info.header_guid)
        .bind(&self.packet_info.packet_directory)
        .bind(&self.packet_info.packet_type)
        .bind(&self.packet_info.delivery_time)
        .bind(&self.packet_info.error)
        .bind(&self.packet_info.default_pdf)
        .bind(&to_json(&self.packet_info.files))
        .bind(&to_json(&self.packet_info.requisites))
        .bind(&to_json(&self.packet_info.sender_info))
        .bind(&self.packet_info.pdf_hash)
        .bind(&self.packet_info.update_key)
        .bind(&to_json(&self.packet_info.acknowledgment))
        .bind(&self.packet_info.visible)
        .bind(&self.packet_info.trace_message)
        .bind(&self.report_sended)
        .execute(&mut c).await?;
        if let Ok(addreesses) = AddresseTable::try_from(&self.packet_info)
        {
            let _ = addreesses.add_or_replace().await;
        }
        Ok(())
    }
//    async fn select<Q: QuerySelector<'a>>(selector: &Q) -> anyhow::Result<Vec<PacketTable>> 
//    {
//         let mut c = get_connection().await?;
//         let query = selector.query();
//         let mut res = query_as::<_, PacketTable>(&query.0);
//         if let Some(params) = query.1
//         {
//             for p in params
//             {
//                 res = res.bind(p);
//             }
//         };
//         let r = res.fetch_all(&mut c)
//         .await?;
//         Ok(r)
//    }

    async fn add_or_replace(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection(Self::base_name()).await?;
        let sql = ["INSERT OR REPLACE INTO ", Self::table_name(), 
        " (
        id,
        task_name,
        header_id,
        directory,
        packet_type,
        delivery_time,
        error,
        default_pdf,
        files,
        requisites,
        sender_info,
        pdf_hash,
        update_key,
        acknowledgment,
        visible,
        trace_message,
        report_sended) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"].concat();
        query(&sql)
        .bind(self.id.to_string())
        .bind(&self.task_name)
        .bind(&self.packet_info.header_guid)
        .bind(&self.packet_info.packet_directory)
        .bind(&self.packet_info.packet_type)
        .bind(&self.packet_info.delivery_time)
        .bind(&self.packet_info.error)
        .bind(&self.packet_info.default_pdf)
        .bind(&to_json(&self.packet_info.files))
        .bind(&to_json(&self.packet_info.requisites))
        .bind(&to_json(&self.packet_info.sender_info))
        .bind(&self.packet_info.pdf_hash)
        .bind(&self.packet_info.update_key)
        .bind(&to_json(&self.packet_info.acknowledgment))
        .bind(&self.packet_info.visible)
        .bind(&self.packet_info.trace_message)
        .bind(&self.report_sended)
        .execute(&mut c).await?;
        if let Ok(addreesses) = AddresseTable::try_from(&self.packet_info)
        {
            let _ = addreesses.add_or_replace().await;
        }
        Ok(())
    }
    async fn add_or_ignore(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection(Self::base_name()).await?;
        let sql = ["INSERT OR IGNORE INTO ", Self::table_name(), 
        " (
        id,
        task_name,
        header_id,
        directory,
        packet_type,
        delivery_time,
        error,
        default_pdf,
        files,
        requisites,
        sender_info,
        pdf_hash,
        update_key,
        acknowledgment,
        visible,
        trace_message,
        report_sended) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"].concat();
        query(&sql)
        .bind(self.id.to_string())
        .bind(&self.task_name)
        .bind(&self.packet_info.header_guid)
        .bind(&self.packet_info.packet_directory)
        .bind(&self.packet_info.packet_type)
        .bind(&self.packet_info.delivery_time)
        .bind(&self.packet_info.error)
        .bind(&self.packet_info.default_pdf)
        .bind(&to_json(&self.packet_info.files))
        .bind(&to_json(&self.packet_info.requisites))
        .bind(&to_json(&self.packet_info.sender_info))
        .bind(&self.packet_info.pdf_hash)
        .bind(&self.packet_info.update_key)
        .bind(&to_json(&self.packet_info.acknowledgment))
        .bind(&self.packet_info.visible)
        .bind(&self.packet_info.trace_message)
        .bind(&self.report_sended)
        .execute(&mut c).await?;
        if let Ok(addreesses) = AddresseTable::try_from(&self.packet_info)
        {
            let _ = addreesses.add_or_replace().await;
        }
        Ok(())
    }
}

impl PacketTable
{
    pub async fn packets_count() -> anyhow::Result<u32>
    {
        //let q = ["SELECT COUNT(*) as count FROM ", Self::table_name()].concat();
        let selector = Selector::new_concat(&["SELECT COUNT(*) as count FROM ", Self::table_name()]);
        let count: CountRequest = Self::get_one(&selector).await?;
        Ok(count.count)
    }
    //TODO добавить выборку по параметрам а не тупо всех подряд, будет и отсеивание по имени и еще по чему то
    ///`rows` - количество записей получаемых из базы данных<br>
    /// `offset` - с какой позиции начинать
    pub async fn get_with_offset(rows: u32, offset: u32, params: Option<Vec<(&str, &str)>>) -> anyhow::Result<Vec<PacketTable>> 
    {
        let ids_offset_selector = Selector::new_concat(&["SELECT id FROM ", Self::table_name()])
        .add_params(params)
        .sort(SortingOrder::Asc("delivery_time"))
        .limit(&rows)
        .offset(&offset);
        let users_ids: Vec<IdSelector> = Self::select_special_type(&ids_offset_selector).await?;
        let id_in = users_ids.into_iter().map(|m| m.0).collect::<Vec<String>>();
        let selector = Selector::new(&Self::full_select())
        .where_in(&id_in)
        .sort(SortingOrder::Asc("delivery_time"));
        let packets = Self::select(&selector).await?;
        Ok(packets)
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
    #[tokio::test]
    async fn test_add_user()
    {
        logger::StructLogger::initialize_logger();
        let paging : Vec<String> = PacketTable::get_with_offset(3, 0, None).await.unwrap().into_iter().map(|m| m.packet_info.delivery_time).collect();
        logger::debug!("{:?}", paging);
    }

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



