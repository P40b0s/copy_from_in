pub enum DbError
{
    Error(String)
}

impl From<rusqlite::Error> for DbError
{
    fn from(value: rusqlite::Error) -> Self 
    {
        match value 
        {
            _ => DbError::Error(format!("{}", value.to_string()))
        }
    }
}