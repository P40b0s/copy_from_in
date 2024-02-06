use std::fmt::Display;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::{DiseaseType, Dictionary};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disease 
{
    pub id: String,
    /**id пользователя */
    pub user_id: String,
    /**тип болезни */
    pub disease_type: DiseaseType,
    pub date_of_illness: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub date_of_recovery: Option<String>,
    pub clinic: Dictionary,
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,
}

impl Disease
{
    pub fn new(id: String, user_id: String, disease_type: DiseaseType, date_of_illness: String, date_of_recovery: Option<String>, clinic: Dictionary, note: Option<String>) -> Self
    {
        Disease {
            id,
            user_id,
            disease_type,
            date_of_illness,
            date_of_recovery,
            clinic,
            note
        }
    }
}
#[cfg(test)]
mod tests
{
    // use uuid::Uuid;

    // #[test]
    // fn test_dis_name()
    // {
    //     let f = super::DiseaseType { id: Uuid::new_v4().to_string(), name: "Covid-19".into(), need_reference: false };
    //     println!("{}", &f);

    // }
}