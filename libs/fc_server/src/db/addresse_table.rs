use db_service::{from_json, get_connection, query, query_as, SqliteRow, to_json, CountRequest, FromRow, Id, Operations, QuerySelector, Result, Row, Selector, SortingOrder};
use logger::backtrace;
use transport::{Ack, PacketInfo, Requisites, SenderInfo};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::contact_info::{ContactInfo, ContactType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddresseTable
{
    pub id: String,
    pub organization: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub medo_addresse: Option<String>,
    pub contact_info: Vec<super::contact_info::ContactInfo>,
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




impl<'a> Id<'a> for AddresseTable
{
    fn get_id(&'a self)-> Uuid
    {
        Uuid::parse_str(&self.id).unwrap()
    }
}


impl FromRow<'_, SqliteRow> for AddresseTable
{
    fn from_row(row: &SqliteRow) -> Result<Self> 
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
    fn base_name() -> &'static str 
    {
        "medo"
    }
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
        let mut c = get_connection(Self::base_name()).await?;
        let sql = ["UPDATE ", Self::table_name(),
        " SET organization = $2,
        medo_addresse = $3,
        contact_info = $4,
        icon = $5
        WHERE id = $1"].concat();
        query(&sql)
        .bind(&self.id)
        .bind(&self.medo_addresse)
        .bind(&to_json(&self.contact_info))
        .bind(&self.icon)
        .execute(&mut c).await?;
        Ok(())
    }
//    async fn select<Q: QuerySelector<'a>>(selector: &Q) -> anyhow::Result<Vec<Self>> 
//    {
//         let mut c = get_connection(base_name()).await?;
//         let query = selector.query();
//         let mut res = sqlx::query_as::<_, Self>(&query.0);
//         if let Some(params) = query.1
//         {
//             for p in params
//             {
//                 res = res.bind(p);
                
//             }
//         };
//         let mut r = res.fetch_all(&mut c)
//         .await?;
//         Ok(r)
//    }

    async fn add_or_replace(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection(Self::base_name()).await?;
        let sql = ["INSERT OR REPLACE INTO ", Self::table_name(), 
        " (id, organization, medo_addresse, contact_info, icon) 
        VALUES ($1, $2, $3, $4, $5)"].concat();
        query(&sql)
        .bind(&self.id)
        .bind(&self.organization)
        .bind(&self.medo_addresse)
        .bind(&to_json(&self.contact_info))
        .bind(&self.icon)
        .execute(&mut c).await?;
        Ok(())
    }
    async fn add_or_ignore(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection(Self::base_name()).await?;
        let sql = ["INSERT OR IGNORE INTO ", Self::table_name(), 
        " (id, organization, medo_addresse, contact_info, icon) 
        VALUES ($1, $2, $3, $4, $5)"].concat();
        query(&sql)
        .bind(&self.id)
        .bind(&self.organization)
        .bind(&self.medo_addresse)
        .bind(&to_json(&self.contact_info))
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
