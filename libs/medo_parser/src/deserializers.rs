use serde::{Deserialize, Deserializer, de::Error};
use uuid::Uuid;

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
