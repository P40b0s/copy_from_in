use serde::{Deserialize, Deserializer, de::Error};
use uuid::Uuid;


// pub fn from_delivery_index<'de, D>(deserializer: D) -> Result<String, D::Error>
// where
// D: Deserializer<'de>,
// {
//     let s: &str = Deserialize::deserialize(deserializer)?;
//     Ok(s.to_lowercase())
// }


// pub fn from_uid<'de, D>(deserializer: D) -> Result<String, D::Error>
// where
// D: Deserializer<'de>,
// {
//     let s: &str = Deserialize::deserialize(deserializer)?;
//     Ok(s.to_lowercase())
// }

// pub fn from_header_type<'de, D>(deserializer: D) -> Result<PacketType, D::Error>
// where
// D: Deserializer<'de>,
// {
//     let s: &str = Deserialize::deserialize(deserializer)?;
//     let t = s.to_lowercase().replace(" ", "");
//     let packet = match t.as_str()
//     {
//         "транспортныйконтейнер" => PacketType::Container,
//         "документ" => PacketType::Document,
//         "письмо" => PacketType::Document,
//         _ => PacketType::Unknown(t)
//     };
//     Ok(packet)
// }

pub fn guid_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
where
D: Deserializer<'de>
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let guid = Uuid::parse_str(&s);
    if let Ok(g) = guid
    {
        Ok(g.to_string())
    }
    else
    {
        return Err(Error::custom(format!("uid {} имеет неверный формат!", guid.err().unwrap().to_string())));
    }
}

#[cfg(test)]
mod tests
{
    use uuid::Uuid;

    #[test]
    fn test_guid()
    {
        let guid = Uuid::parse_str("032d8450-f246-4182-4725-88470022a591");
        println!("{}", guid.unwrap());
    }
}
