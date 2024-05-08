use std::borrow::Cow;

// use super::{contact_info::{ContactInfo, ContactType}, from_json, get_connection};
// use medo_parser::PacketInfo;
// //use rusqlite::{Result, params, Error, ToSql, Rows, Params};
// use serde::{Serialize, Deserialize};
// use serde_json::json;
use logger::backtrace;
use transport::{Ack, PacketInfo, Requisites, SenderInfo};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Row, sqlite::SqliteRow, FromRow, Execute};

use super::{connection::get_connection, contact_info::ContactType, from_json, operations::{CountRequest, Id, IdSelector, Operations, QuerySelector, Selector, SortingOrder}, ContactInfo};
//id TEXT PRIMARY KEY NOT NULL, 


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddresseTable
{
    pub id: String,
    pub organization: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub medo_addresse: Option<String>,
    pub contact_info: Vec<ContactInfo>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon: Option<String>,
}

impl TryFrom<&PacketInfo> for AddresseTable
{
    type Error = String;
    fn try_from(value: &PacketInfo) -> Result<Self, Self::Error> 
    {
        
        let id = value.sender_info.as_ref().and_then(|s| s.source_guid.as_ref().cloned()).ok_or("id организации отправителя не найден".to_owned())?;
        let organization = value.sender_info.as_ref().and_then(|s| s.organization.as_ref().cloned()).ok_or("наименование организации отправителя не найдено".to_owned())?;
        let medo_addresse = value.sender_info.as_ref().and_then(|s| s.medo_addressee.as_ref().cloned());
        //Сбор контактных данных
        let executor = value.sender_info.as_ref().and_then(|e| e.executor.as_ref().cloned());
        let mut contacts: Vec<ContactInfo> = vec![];
        if let Some(executor) = executor
        {
            let def = "".to_owned();
            let org = executor.organization.as_ref().unwrap_or(&def);
            let person = executor.person.as_ref().unwrap_or(&def);
            let post = executor.post.as_ref().unwrap_or(&def);
            let cont = executor.contact_info.as_ref().unwrap_or(&def);
            if (org.len() + person.len() + post.len() + cont.len())  > 0
            {
                let hash = utilites::Hasher::hash_from_strings(&[org, person, post, cont]);
                let mut ct: Vec<ContactType> = vec![];
                if cont.len() > 0
                {
                    let c = ContactType
                    {
                        contact_type: String::from("телефон"),
                        value: cont.clone()
                    };
                    ct.push(c);
                }
                let contact = ContactInfo
                {
                    id : Some(hash),
                    organization: Some(org.clone()),
                    person: Some(person.clone()),
                    post: Some(post.clone()),
                    photo: None,
                    contacts: ct,
                    note: None
                };
                contacts.push(contact);
            }
        }
        Ok(AddresseTable
        {
            id,
            organization,
            medo_addresse,
            icon: None,
            contact_info: contacts,
        })
    }
}


// impl AddresseTable
// {
//     pub fn select_by_medo_addr(addr: &str) -> Result<Box<Self>> 
//     {
//         let c = get_connection()?;
//         let mut stmt = c.prepare("SELECT id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key FROM addresses WHERE medo_addresse = ?")?;
//         let mut rows = stmt.query(params![addr])?;
//         let mut addresse: AddresseTable = AddresseTable::default();
//         while let Some(row) = rows.next()?
//         {
//             addresse.id = row.get(0)?;
//             addresse.organization = row.get(1)?;
//             addresse.medo_addresse = row.get(2)?;
//             let notify_addresses: String = row.get(3)?;
//             if let Ok(ntf) =  serde_json::from_str::<Vec<String>>(&notify_addresses)
//             {
//                 addresse.notifications_sources_medo_addresses = ntf;
//             }
//             addresse.icon = row.get(4)?;
//             let executor: String  = row.get(5)?;
//             if let Ok(ex) = serde_json::from_str::<Vec<ContactInfo>>(&executor)
//             {
//                 addresse.contact_info = ex;
//             }
//             addresse.update_key = row.get(6)?;
//             return Ok(Box::new(addresse));   
//         }
//         return Err(Error::QueryReturnedNoRows);
//     }

// }

// impl DbInterface for AddresseTable
// {
//     const SELECT_BODY: &'static str = "SELECT id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key FROM addresses";
//     fn create() -> Result<()>
//     {
//         let c = get_connection()?;
//             c.execute(
//                 "CREATE TABLE IF NOT EXISTS addresses (
//                     id TEXT PRIMARY KEY NOT NULL, 
//                     organization TEXT, 
//                     medo_addresse TEXT, 
//                     notifications_sources_medo_addresses JSON DEFAULT('[]'),
//                     contact_info JSON DEFAULT('[]'),
//                     update_key TEXT,
//                     icon BLOB
//                     );",
//                 (),
//             )?;
//         Ok(())
//     }

//     fn add_or_replace(&self) -> Result<()>
//     {
//         let c = get_connection()?;
//             c.execute(
//                 "INSERT OR REPLACE INTO addresses (id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
//                 (&self.id, &self.organization, &self.medo_addresse, json!(&self.notifications_sources_medo_addresses), &self.icon, json!(&self.contact_info), &self.update_key),
//             )?;
//         Ok(())
//     }
//     fn add_or_ignore(&self) -> Result<()>
//     {
//         let c = get_connection()?;
//             c.execute(
//                 "INSERT OR IGNORE INTO addresses (id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
//                 (&self.id, &self.organization, &self.medo_addresse, json!(&self.notifications_sources_medo_addresses), &self.icon, json!(&self.contact_info), &self.update_key),
//             )?;
//         Ok(())
//     }
//     fn delete(&self) -> Result<()>
//     {
//         let c = get_connection()?;
//         c.execute(
//             "DELETE FROM addresses WHERE id = ?1",[&self.id]
//         )?;
//         Ok(())
//     }
//     fn update(&self) -> Result<()>
//     {
//         let c = get_connection()?;
//             c.execute(
//                 "UPDATE addresses SET organization = ?2, medo_addresse = ?3, notifications_sources_medo_addresses = ?4, icon = ?5, contact_info = ?6, update_key = ?7  WHERE id = ?1",
//                 (&self.id, &self.organization, &self.medo_addresse, json!(&self.notifications_sources_medo_addresses), &self.icon, json!(&self.contact_info), &self.update_key),
//             )?;
//         Ok(())
//     }
//     fn select(id: &str) -> Result<Box<Self>> 
//     {
//         let result = Self::query(Some("id = :id"), &[(":id", id)])?;
//         if let Some(r) = result.into_iter().nth(0)
//         {
//             return Ok(r)
//         }
//         else
//         {
//             return Err(Error::QueryReturnedNoRows);
//         }
//     }

//     fn select_body_query(rows: &mut Rows) -> Result<Vec<Box<AddresseTable>>>
//     {
//         let mut results: Vec<Box<AddresseTable>> = vec![];
//         while let Some(row) = rows.next()?
//         {
//             let mut addresse: AddresseTable = AddresseTable::default();
//             addresse.id = row.get(0)?;
//             addresse.organization = row.get(1)?;
//             addresse.medo_addresse = row.get(2)?;
//             let notify_addresses: String = row.get(3)?;
//             if let Ok(ntf) =  serde_json::from_str::<Vec<String>>(&notify_addresses)
//             {
//                 addresse.notifications_sources_medo_addresses = ntf;
//             }
//             addresse.icon = row.get(4)?;
//             let executor: String  = row.get(5)?;
//             if let Ok(ex) = serde_json::from_str::<Vec<ContactInfo>>(&executor)
//             {
//                 addresse.contact_info = ex;
//             }
//             addresse.update_key = row.get(6)?;
//             results.push(Box::new(addresse));
//         }
//     return Ok(results);
//     }

//     fn drop(clean: bool) -> Result<()>
//     {
//         let c = get_connection()?;
//         if clean
//         {
//             c.execute(
//                 "DELETE FROM addresses",[]
//             )?;
//         }
//         else 
//         {
//             c.execute(
//                 "DROP TABLE addresses",
//                 (),
//             )?;
//         }
//         Ok(())
//     }
// }

// impl super::BatchOperation for Vec<AddresseTable> 
// {
//     fn batch(&self) -> Result<()>  where
//         Self: Default + Sized
//     {
//         let mut c = get_connection()?;
//         let mut tx = c.transaction()?;
//         for a in self
//         {
//             tx.execute(
//                 "INSERT OR IGNORE INTO addresses (id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
//                 (&a.id, &a.organization, &a.medo_addresse, json!(&a.notifications_sources_medo_addresses), &a.icon, json!(&a.contact_info), &a.update_key),
//             )?;
//         }
//         tx.commit()?;
//         Ok(())
//     }    
// }


impl<'a> Id<'a> for AddresseTable
{
    fn get_id(&'a self)-> Cow<str> 
    {
        Cow::from(&self.id)
    }
}


impl FromRow<'_, SqliteRow> for AddresseTable
{
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
    {
        Ok(Self 
        {
            id: row.try_get("id")?,
            organization: row.try_get("organization")?,
            medo_addresse: row.try_get("medo_addresse")?,
            contact_info: from_json(row, "contact_info").unwrap_or(vec![]),
            icon: row.try_get("icon")?
        })
    }
}

impl<'a> Operations<'a> for AddresseTable
{
    fn table_name() -> &'static str 
    {
       "addresses"
    }
    fn create_table() -> String 
    {  
        ["CREATE TABLE IF NOT EXISTS ", Self::table_name(), " (
            id TEXT PRIMARY KEY NOT NULL, 
            organization TEXT NOT NULL, 
            medo_addresse TEXT, 
            contact_info JSON DEFAULT('[]'),
            icon BLOB
            );"].concat()
    }
    fn full_select() -> String 
    {
        ["SELECT 
        id, 
        organization, 
        medo_addresse,
        contact_info,
        icon
        FROM ", Self::table_name()].concat()
    }
    async fn update(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let sql = ["UPDATE ", Self::table_name(),
        " SET organization = $2,
        medo_addresse = $3,
        contact_info = $4,
        icon = $5
        WHERE id = $1"].concat();
        sqlx::query(&sql)
        .bind(&self.id)
        .bind(&self.medo_addresse)
        .bind(&json!(&self.contact_info))
        .bind(&self.icon)
        .execute(&mut c).await?;
        Ok(())
    }
   async fn select<Q: QuerySelector<'a>>(selector: &Q) -> anyhow::Result<Vec<Self>> 
   {
        let mut c = get_connection().await?;
        let query = selector.query();
        let mut res = sqlx::query_as::<_, Self>(&query.0);
        if let Some(params) = query.1
        {
            for p in params
            {
                res = res.bind(p);
                
            }
        };
        let mut r = res.fetch_all(&mut c)
        .await?;
        Ok(r)
   }

    async fn add_or_replace(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let sql = ["INSERT OR REPLACE INTO ", Self::table_name(), 
        " (id, organization, medo_addresse, contact_info, icon) 
        VALUES ($1, $2, $3, $4, $5)"].concat();
        sqlx::query(&sql)
        .bind(&self.id)
        .bind(&self.organization)
        .bind(&self.medo_addresse)
        .bind(&json!(&self.contact_info))
        .bind(&self.icon)
        .execute(&mut c).await?;
        Ok(())
    }
    async fn add_or_ignore(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let sql = ["INSERT OR IGNORE INTO ", Self::table_name(), 
        " (id, organization, medo_addresse, contact_info, icon) 
        VALUES ($1, $2, $3, $4, $5)"].concat();
        sqlx::query(&sql)
        .bind(&self.id)
        .bind(&self.organization)
        .bind(&self.medo_addresse)
        .bind(&json!(&self.contact_info))
        .bind(&self.icon)
        .execute(&mut c).await?;
        Ok(())
    }
}

impl AddresseTable
{
    pub async fn count() -> anyhow::Result<u32>
    {
        let q = ["SELECT COUNT(*) as count FROM ", Self::table_name()].concat();
        let selector = Selector::new(&q);
        let count: CountRequest = Self::get_one(&selector).await?;
        Ok(count.count)
    }
}
