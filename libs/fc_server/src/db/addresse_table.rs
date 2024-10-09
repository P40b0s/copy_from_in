use std::sync::Arc;

use db_service::{from_json, get_fields_for_update, query,  to_json, CountRequest, DbError, FromRow, QuerySelector, Result, Row, Selector, SqlOperations, SqlitePool, SqliteRow};
use transport::PacketInfo;
use serde::{Deserialize, Serialize};

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

impl<'a> SqlOperations<'a> for AddresseTable
{
    fn get_id(&'a self) -> &'a str
    {
        &self.id
    }
    fn table_name() -> &'static str 
    {
       "addresses"
    }
    fn table_fields() -> &'a[&'static str]
    {
        &[
            "id", //0
            "organization", //1
            "medo_addresse", //2
            "contact_info", //3
            "icon", //4
        ]
    }
    fn create_table() -> String 
    {  
        ["CREATE TABLE IF NOT EXISTS ", Self::table_name(), " (
            ", Self::table_fields()[0], " TEXT PRIMARY KEY NOT NULL, 
            ", Self::table_fields()[1], " TEXT NOT NULL, 
            ", Self::table_fields()[2], " TEXT, 
            ", Self::table_fields()[3], " JSON DEFAULT('[]'),
            ", Self::table_fields()[4], " BLOB
            );"].concat()
    }

    async fn update(&'a self, pool: Arc<SqlitePool>) -> Result<(), DbError>
    {
        let update_set = get_fields_for_update(Self::table_fields());
        let sql = ["UPDATE ", Self::table_name(),
        " SET ", &update_set ," WHERE ", Self::table_fields()[0]," = $1"].concat();

        query(&sql)
        .bind(self.id.to_string())
        .bind(&self.organization)
        .bind(&self.medo_addresse)
        .bind(to_json(&self.contact_info))
        .bind(self.icon.as_ref())
        .execute(&*pool).await?;
        Ok(())
    }

    async fn add_or_replace(&'a self, pool: Arc<SqlitePool>) -> Result<(), DbError>
    {
        let sql = Self::insert_or_replace_query();
        query(&sql)
        .bind(self.id.to_string())
        .bind(&self.organization)
        .bind(&self.medo_addresse)
        .bind(to_json(&self.contact_info))
        .bind(self.icon.as_ref())
        .execute(&*pool).await?;
        Ok(())
    }
    async fn add_or_ignore(&'a self, pool: Arc<SqlitePool>) -> Result<(), DbError>
    {
        let sql = Self::insert_or_ignore_query();
        query(&sql)
        .bind(self.id.to_string())
        .bind(&self.organization)
        .bind(&self.medo_addresse)
        .bind(to_json(&self.contact_info))
        .bind(self.icon.as_ref())
        .execute(&*pool).await?;
        Ok(())
    }
    
}

impl AddresseTable
{
    pub async fn count(pool: Arc<SqlitePool>) -> Result<u32, DbError>
    {
        let q = ["SELECT COUNT(*) as count FROM ", Self::table_name()].concat();
        let selector = Selector::new(&q);
        let count: CountRequest = Self::get_one(&selector, pool).await?;
        Ok(count.count)
    }
}
