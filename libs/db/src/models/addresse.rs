use super::{get_connection, DbInterface, contact_info::{ContactInfo, ContactType}};
use medo_parser::PacketInfo;
use rusqlite::{Result, params, Error, ToSql, Rows, Params};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddresseTable
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub medo_addresse: Option<String>,
    pub contact_info: Vec<ContactInfo>,
    pub notifications_sources_medo_addresses: Vec<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub update_key: Option<String>,
}

impl From<&PacketInfo> for AddresseTable
{
    fn from(value: &PacketInfo) -> Self 
    {
        let id = value.sender_info.as_ref().and_then(|s| s.source_guid.as_ref().cloned());
        let organization = value.sender_info.as_ref().and_then(|s| s.organization.as_ref().cloned());
        let medo_addresse = value.sender_info.as_ref().and_then(|s| s.medo_addessee.as_ref().cloned());
        let mut notify: Vec<String> = vec![];
        if medo_addresse.is_some()
        {
            notify.push(medo_addresse.as_ref().unwrap().clone())
        }
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
                let hash = utilites::Hasher::hash_from_string(&[org, person, post, cont]);
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


        AddresseTable
        {
            id,
            organization,
            medo_addresse,
            notifications_sources_medo_addresses: notify,
            icon: None,
            contact_info: contacts,
            update_key: None,
        }
    }
}

impl Default for AddresseTable
{
    fn default() -> Self 
    {
        AddresseTable
        {
            id: None,
            medo_addresse: None,
            organization: None,
            notifications_sources_medo_addresses: vec![],
            icon: None,
            contact_info: vec![],
            update_key: None,
        }
    }
}

impl AddresseTable
{
    pub fn select_by_medo_addr(addr: &str) -> Result<Box<Self>> 
    {
        let c = get_connection()?;
        let mut stmt = c.prepare("SELECT id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key FROM addresses WHERE medo_addresse = ?")?;
        let mut rows = stmt.query(params![addr])?;
        let mut addresse: AddresseTable = AddresseTable::default();
        while let Some(row) = rows.next()?
        {
            addresse.id = row.get(0)?;
            addresse.organization = row.get(1)?;
            addresse.medo_addresse = row.get(2)?;
            let notify_addresses: String = row.get(3)?;
            if let Ok(ntf) =  serde_json::from_str::<Vec<String>>(&notify_addresses)
            {
                addresse.notifications_sources_medo_addresses = ntf;
            }
            addresse.icon = row.get(4)?;
            let executor: String  = row.get(5)?;
            if let Ok(ex) = serde_json::from_str::<Vec<ContactInfo>>(&executor)
            {
                addresse.contact_info = ex;
            }
            addresse.update_key = row.get(6)?;
            return Ok(Box::new(addresse));   
        }
        return Err(Error::QueryReturnedNoRows);
    }

}

impl DbInterface for AddresseTable
{
    const SELECT_BODY: &'static str = "SELECT id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key FROM addresses";
    fn create() -> Result<()>
    {
        let c = get_connection()?;
            c.execute(
                "CREATE TABLE IF NOT EXISTS addresses (
                    id TEXT PRIMARY KEY NOT NULL, 
                    organization TEXT, 
                    medo_addresse TEXT, 
                    notifications_sources_medo_addresses JSON DEFAULT('[]'),
                    contact_info JSON DEFAULT('[]'),
                    update_key TEXT,
                    icon BLOB
                    );",
                (),
            )?;
        Ok(())
    }

    fn add_or_replace(&self) -> Result<()>
    {
        let c = get_connection()?;
            c.execute(
                "INSERT OR REPLACE INTO addresses (id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&self.id, &self.organization, &self.medo_addresse, json!(&self.notifications_sources_medo_addresses), &self.icon, json!(&self.contact_info), &self.update_key),
            )?;
        Ok(())
    }
    fn add_or_ignore(&self) -> Result<()>
    {
        let c = get_connection()?;
            c.execute(
                "INSERT OR IGNORE INTO addresses (id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&self.id, &self.organization, &self.medo_addresse, json!(&self.notifications_sources_medo_addresses), &self.icon, json!(&self.contact_info), &self.update_key),
            )?;
        Ok(())
    }
    fn delete(&self) -> Result<()>
    {
        let c = get_connection()?;
        c.execute(
            "DELETE FROM addresses WHERE id = ?1",[&self.id]
        )?;
        Ok(())
    }
    fn update(&self) -> Result<()>
    {
        let c = get_connection()?;
            c.execute(
                "UPDATE addresses SET organization = ?2, medo_addresse = ?3, notifications_sources_medo_addresses = ?4, icon = ?5, contact_info = ?6, update_key = ?7  WHERE id = ?1",
                (&self.id, &self.organization, &self.medo_addresse, json!(&self.notifications_sources_medo_addresses), &self.icon, json!(&self.contact_info), &self.update_key),
            )?;
        Ok(())
    }
    fn select(id: &str) -> Result<Box<Self>> 
    {
        let result = Self::query(Some("id = :id"), &[(":id", id)])?;
        if let Some(r) = result.into_iter().nth(0)
        {
            return Ok(r)
        }
        else
        {
            return Err(Error::QueryReturnedNoRows);
        }
    }

    fn select_body_query(rows: &mut Rows) -> Result<Vec<Box<AddresseTable>>>
    {
        let mut results: Vec<Box<AddresseTable>> = vec![];
        while let Some(row) = rows.next()?
        {
            let mut addresse: AddresseTable = AddresseTable::default();
            addresse.id = row.get(0)?;
            addresse.organization = row.get(1)?;
            addresse.medo_addresse = row.get(2)?;
            let notify_addresses: String = row.get(3)?;
            if let Ok(ntf) =  serde_json::from_str::<Vec<String>>(&notify_addresses)
            {
                addresse.notifications_sources_medo_addresses = ntf;
            }
            addresse.icon = row.get(4)?;
            let executor: String  = row.get(5)?;
            if let Ok(ex) = serde_json::from_str::<Vec<ContactInfo>>(&executor)
            {
                addresse.contact_info = ex;
            }
            addresse.update_key = row.get(6)?;
            results.push(Box::new(addresse));
        }
    return Ok(results);
    }

    fn drop(clean: bool) -> Result<()>
    {
        let c = get_connection()?;
        if clean
        {
            c.execute(
                "DELETE FROM addresses",[]
            )?;
        }
        else 
        {
            c.execute(
                "DROP TABLE addresses",
                (),
            )?;
        }
        Ok(())
    }
}

impl super::BatchOperation for Vec<AddresseTable> 
{
    fn batch(&self) -> Result<()>  where
        Self: Default + Sized
    {
        let mut c = get_connection()?;
        let mut tx = c.transaction()?;
        for a in self
        {
            tx.execute(
                "INSERT OR IGNORE INTO addresses (id, organization, medo_addresse, notifications_sources_medo_addresses, icon, contact_info, update_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (&a.id, &a.organization, &a.medo_addresse, json!(&a.notifications_sources_medo_addresses), &a.icon, json!(&a.contact_info), &a.update_key),
            )?;
        }
        tx.commit()?;
        Ok(())
    }    
}
