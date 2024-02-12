use serde::{Serialize, Deserialize, Deserializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename="destination")]
///Появился с версии 2.7.1
pub struct DeliveryIndex
{
    destination: Vec<Destinations>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(rename="destination")]
///Появился с версии 2.7.1
pub struct Destinations
{
    destination: Destination,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Destination
{
    #[serde(rename="@uid")]
    uid: Option<String>,
    organization: Option<String>,
}


pub fn from_delivery_index<'de, D>(deserializer: D) -> Result<Destination, D::Error>
where
D: Deserializer<'de>,
{
    let s: Destination = Deserialize::deserialize(deserializer)?;
    Ok(s)
}



// use serde::{Serialize, Deserialize};
// use super::Organization;

// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Addressee
// {
//     organization: Organization
// }


// #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Addresseess
// {
//     #[serde(rename="addressee")]
//     addresseess: Vec<Addressee>
// }

