use std::{slice::Iter, time::SystemTime};

use serde::Serialize;
use serde_json::json;
use universal_interface::{PacketInfo, Ack, PublicationInfo};
use super::{Requisites, SenderInfo, get_connection, DbInterface, AddresseTable};
use rusqlite::{Result, params, Error, Statement, Rows, Params, Transaction};
use uuid::Uuid;

///Для кастомных запросов видимо придется реализовать отдельные структуры
#[derive(Serialize, Clone)]
pub struct PacketTypesCount
{
    count: u32,
    packet_type: String
}
impl Default for PacketTypesCount
{
    fn default() -> Self 
    {
        PacketTypesCount { count: 0, packet_type: "Неизвестно".to_owned() }
    }
}

impl PacketTypesCount
{
    pub fn get_packets_types() -> Result<Vec<PacketTypesCount>>
    {
        let res = universal_interface::PacketInfo::custom_query(
            "SELECT COUNT(*) as count, packet_type FROM packets GROUP BY packet_type",
            |f|
            {
                let mut rows: Vec<PacketTypesCount> = vec![];
                while let Some(row) = f.next()?
                {
                    let mut packet_count = PacketTypesCount::default();
                    packet_count.count = row.get(0)?;
                    packet_count.packet_type = row.get(1)?;
                    rows.push(packet_count);
                }
                return Ok(rows);
            },
            []);
        return res;
    }
}

impl DbInterface for universal_interface::PacketInfo
{
    const SELECT_BODY: &'static str = "SELECT header_id, directory, packet_type, delivery_time, error, default_pdf, files, requisites, sender_info, pdf_hash, update_key, acknowledgment, visible, trace_message, publication_info FROM packets";
    fn create() -> Result<()>
    {
        let c = get_connection()?;
            c.execute(
                "CREATE TABLE IF NOT EXISTS packets (
                    header_id TEXT PRIMARY KEY NOT NULL, 
                    directory TEXT, 
                    packet_type TEXT,
                    delivery_time TEXT,
                    error TEXT,
                    default_pdf TEXT, 
                    pdf_hash TEXT,
                    files JSON DEFAULT('[]'),
                    requisites JSON,
                    sender_info JSON,
                    acknowledgment JSON,
                    update_key TEXT,
                    visible INTEGER NOT NULL DEFAULT 1,
                    trace_message TEXT,
                    publication_info JSON
                    );",
                (),
            )?;
        Ok(())
    }

    fn add_or_replace(&self) -> Result<()>
    {
        let uid = self.header_guid.as_ref().cloned().or(Some(Uuid::new_v4().to_string()));
        let c = get_connection()?;
            c.execute(
                "INSERT OR REPLACE INTO packets (header_id, directory, packet_type, delivery_time, error, default_pdf, files, requisites, sender_info, pdf_hash, update_key, acknowledgment, visible, trace_message, publication_info) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                (uid, &self.packet_directory, &self.packet_type, &self.delivery_time, &self.error, &self.default_pdf, json!(&self.files), json!(&self.requisites), json!(&self.sender_info), &self.pdf_hash, &self.update_key, json!(&self.acknowledgment), &self.visible, &self.trace_message, json!(&self.publication_info)),
            )?;
        if self.sender_info.is_some()
        {
            let addr : AddresseTable = self.into();
            addr.add_or_ignore()?;
        }
        Ok(())
    }

    fn add_or_ignore(&self) -> Result<()>
    {
        let uid = self.header_guid.as_ref().cloned().or(Some(Uuid::new_v4().to_string()));
        let c = get_connection()?;
        c.execute(
            "INSERT OR IGNORE INTO packets (header_id, directory, packet_type, delivery_time, error, default_pdf, files, requisites, sender_info, pdf_hash, update_key, acknowledgment, visible, trace_message, publication_info) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            (uid, &self.packet_directory, &self.packet_type, &self.delivery_time, &self.error, &self.default_pdf, json!(&self.files), json!(&self.requisites), json!(&self.sender_info), &self.pdf_hash, &self.update_key, json!(&self.acknowledgment), &self.visible, &self.trace_message, json!(&self.publication_info)),
        )?;
        if self.sender_info.is_some()
        {
            let addr : AddresseTable = self.into();
            addr.add_or_ignore()?;
            let uid_err = String::from("id отправителя не найден");
            let org_err = String::from("организация не найдена");
            let addr_err = String::from("Адрес МЭДО не найден");
            let uid = self.sender_info.as_ref().unwrap().source_guid.as_ref().unwrap_or(&uid_err);
            let org = self.sender_info.as_ref().unwrap().organization.as_ref().unwrap_or(&org_err);
            let addr = self.sender_info.as_ref().unwrap().medo_addessee.as_ref().unwrap_or(&addr_err);
            logger::debug!("Транзакция по пакету {} (id:{} от-> {}<{}>) завершена",&self.packet_directory, uid, org, addr);
        }
        logger::debug!("Транзакция по пакету {} завершена",&self.packet_directory);
        Ok(())
    }
    fn delete(&self) -> Result<()>
    {
        let c = get_connection()?;
        c.execute(
            "DELETE FROM packets WHERE header_id = ?1",[&self.header_guid]
        )?;
        Ok(())
    }
    fn update(&self) -> Result<()>
    {
        let update_date = medo_settings::convert_system_time(SystemTime::now());
        let c = get_connection()?;
            c.execute(
                "UPDATE packets SET directory = ?2, packet_type = ?3, delivery_time = ?4, error = ?5, default_pdf = ?6, files = ?7, requisites = ?8, sender_info = ?9, pdf_hash = ?10, update_key = ?11, acknowledgment = ?12, visible = ?13, trace_message = ?14, publication_info = ?15 WHERE header_id = ?1",
                (&self.header_guid, &self.packet_directory, &self.packet_type, &self.delivery_time, &self.error, &self.default_pdf, json!(&self.files), json!(&self.requisites), json!(&self.sender_info), &self.pdf_hash, update_date, json!(&self.acknowledgment), &self.visible, &self.trace_message, json!(&self.publication_info)),
            )?;
        Ok(())
    }

    fn select(id: &str) -> Result<Box<Self>> 
    {
        let result = Self::query(Some("header_id = :header_id"), &[(":header_id", id)])?;
        if let Some(r) = result.into_iter().nth(0)
        {
            return Ok(r)
        }
        else
        {
            return Err(Error::QueryReturnedNoRows);
        }
    }
    
    // fn query<P>(where_params: &str, params: P) -> Result<Vec<Box<PacketInfo>>> where P : Params 
    // {
    //     let c = get_connection()?;
    //     let create_body = [Self::SELECT_BODY, " WHERE ", where_params].concat();
    //     let mut stmt = c.prepare(&create_body)?;
    //     let mut rows = stmt.query(params)?;
    //     body_query(&mut rows)
    // }

    //SELECT header_id, directory, packet_type, delivery_time, error, files, requisites, sender_info, default_pdf FROM packets
    fn select_body_query(rows: &mut Rows) -> Result<Vec<Box<PacketInfo>>>
    {
        let mut results: Vec<Box<PacketInfo>> = vec![];
        while let Some(row) = rows.next()?
        {
            let mut packet = universal_interface::PacketInfo::default();
            packet.header_guid = row.get(0)?;
            packet.packet_directory = row.get(1)?;
            packet.packet_type = row.get(2)?;
            packet.delivery_time = row.get(3)?;
            packet.error = row.get(4)?;
            packet.default_pdf = row.get(5)?;
            let files: Option<String> = row.get(6)?;
            if files.is_some()
            {
                if let Ok(f) =  serde_json::from_str::<Vec<String>>(files.as_ref().unwrap())
                {
                    packet.files = f;
                }
            }
            let req: Option<String> = row.get(7)?;
            if let Ok(r) =  serde_json::from_str::<Requisites>(req.as_ref().unwrap_or(&"".to_owned()))
            {
                packet.requisites = Some(r);
            }
            let send: Option<String> = row.get(8)?;
            if let Ok(s) =  serde_json::from_str::<SenderInfo>(send.as_ref().unwrap_or(&"".to_owned()))
            {
                packet.sender_info = Some(s);
            }
            packet.pdf_hash = row.get(9)?;
            packet.update_key = row.get(10)?;
            //10 это update_key он тут не нужен
            let ack: Option<String> = row.get(11)?;
            if ack.is_some()
            {
                if let Ok(a) =  serde_json::from_str::<Ack>(ack.as_ref().unwrap())
                {
                    packet.acknowledgment = Some(a);
                }
            }
            packet.visible = row.get(12)?;
            packet.trace_message = row.get(13)?;
            let pinfo: Option<String> = row.get(14)?;
            if pinfo.is_some()
            {
                if let Ok(p) =  serde_json::from_str::<PublicationInfo>(pinfo.as_ref().unwrap())
                {
                    packet.publication_info = Some(p);
                }
            }
            results.push(Box::new(packet));
        }
        return Ok(results);
    }

    fn drop(clean: bool) -> Result<()>
    {
        let c = get_connection()?;
        if clean
        {
            c.execute(
                "DELETE FROM packets",[]
            )?;
        }
        else 
        {
            c.execute(
                "DROP TABLE packets",
                (),
            )?;
        } 
        Ok(())
    }
}

// impl DbSelectionInterface for PacketInfo
// {
//     const BODY: &'static str = "SELECT header_id, directory, packet_type, delivery_time, error, default_pdf, files, requisites, sender_info FROM packets WHERE ";
//     fn query<P>(where_params: &str, params: P) -> Result<Vec<Box<PacketInfo>>> where P : Params 
//     {
//         let c = get_connection()?;
//         let create_body = [PacketInfo::BODY, where_params].concat();
//         let mut stmt = c.prepare(&create_body)?;
//         let mut rows = stmt.query(params)?;
//         body_query(&mut rows)
//     }
// }

impl super::BatchOperation for Vec<PacketInfo> 
{
    fn batch(&self) -> Result<()>  where
        Self: Default + Sized
    {
        if self.len() == 0
        {
            return Ok(());
        }
        let update_date = medo_settings::convert_system_time(SystemTime::now());
        let mut c = get_connection()?;
        let mut tx = c.transaction()?;
        let mut addresses: Vec<AddresseTable> = vec![];
        for p in self
        {
            let uid = p.header_guid.as_ref().cloned().or(Some(Uuid::new_v4().to_string()));
            tx.execute(
                "INSERT OR REPLACE INTO packets (header_id, directory, packet_type, delivery_time, error, default_pdf, files, requisites, sender_info, pdf_hash, update_key, acknowledgment, visible, trace_message, publication_info) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                (uid, &p.packet_directory, &p.packet_type, &p.delivery_time, &p.error, &p.default_pdf, json!(p.files), json!(p.requisites), json!(p.sender_info), &p.pdf_hash, &update_date, json!(&p.acknowledgment), &p.visible, &p.trace_message, json!(&p.publication_info)),
            )?;
            if p.sender_info.is_some()
            {
                addresses.push(p.into());
            }
        }
        logger::debug!("Идет выполнение транзакции в базе данных, ожидайте....");
        tx.commit()?;
        addresses.batch()?;
        logger::debug!("Выполнение транзакции завершено");
        Ok(())
    }    
}
// // fn batch_packets() -> Result<()>
// // {
// //     let mut c = get_connection()?;
// //     let mut tx = c.transaction()?;
// //     for p in packets
// //     {
// //         let uid = p.header_guid.as_ref().cloned().or(Some(Uuid::new_v4().to_string()));
// //         tx.execute(
// //             "INSERT OR IGNORE INTO packets (header_id, directory, packet_type, delivery_time, error, default_pdf, files, requisites, sender_info) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
// //             (uid, &p.packet_directory, &p.packet_type, &p.delivery_time, &p.error, &p.default_pdf, json!(p.files), json!(p.requisites), json!(p.sender_info)),
// //         )?;
// //         logger::debug!("Пакет {} добавлен для выполнения транзакции",p.packet_directory);
// //     }
// //     tx.commit()?;
// //     Ok(())
// // }

// fn select_body_query(rows: &mut Rows) -> Result<Vec<Box<PacketInfo>>>
// {
//     let mut results: Vec<Box<PacketInfo>> = vec![];
//     while let Some(row) = rows.next()?
//     {
//         let mut packet = universal_interface::PacketInfo::default();
//         packet.header_guid = row.get(0)?;
//         packet.packet_directory = row.get(1)?;
//         packet.packet_type = row.get(2)?;
//         packet.delivery_time = row.get(3)?;
//         packet.error = row.get(4)?;
//         packet.default_pdf = row.get(5)?;
//         let files: String = row.get(6)?;
//         if let Ok(f) =  serde_json::from_str::<Vec<String>>(&files)
//         {
//             packet.files = f;
//         }
//         let req: String = row.get(7)?;
//         if let Ok(r) =  serde_json::from_str::<Requisites>(&req)
//         {
//             packet.requisites = Some(r);
//         }
//         let send: String = row.get(8)?;
//         if let Ok(s) =  serde_json::from_str::<SenderInfo>(&send)
//         {
//             packet.sender_info = Some(s);
//         }
//         results.push(Box::new(packet));
//     }
//     return Ok(results);
//}

