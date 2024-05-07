pub enum DbError
{
    Error(String)
}

impl From<sqlx::Error> for DbError
{
    fn from(value: sqlx::Error) -> Self 
    {
        match value 
        {
            _ => DbError::Error(format!("{}", value.to_string()))
        }
    }
}