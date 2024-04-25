use utilites::{Date, DateFormat};

use crate::Error;


pub async fn get_date_now() -> Result<String, Error>
{
    let date = Date::now();
    Ok(date.format(DateFormat::Serialize))
}
