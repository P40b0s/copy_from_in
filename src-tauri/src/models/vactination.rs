use std::fmt::Display;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::DiseaseType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vactination
{
    id: String,
    disease_type: DiseaseType,
    date: String,
    ///Особая отметка
    note: String
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