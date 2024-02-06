use std::fmt::Display;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dictionary
{
    pub id: String,
    pub name: String,
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