use crate::helpers::{Date, DateTimeFormat, DateFormat};
use crate::Error;


pub async fn get_date_now() -> Result<String, Error>
{
    let date = Date::now();
    Ok(date.write(DateFormat::Serialize))
}
