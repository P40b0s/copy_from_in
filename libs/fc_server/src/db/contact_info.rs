use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactType
{
    pub contact_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactInfo
{
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub person: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub post: Option<String>,
    pub contacts: Vec<ContactType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub photo: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    ///Поле введено отдельно, в него информация при парсинге не поступает, это для фронта
    pub note: Option<String>
}
impl Default for ContactInfo
{
    fn default() -> Self 
    {
        ContactInfo 
        { 
            id: None,
            organization: None,
            person: None,
            post: None,
            contacts: vec![],
            photo: None,
            note: None
        }
    }
}