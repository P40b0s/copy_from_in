mod directories_spy;
pub use  directories_spy::DirectoriesSpy;
use medo_parser::Packet;
use settings::DateTimeFormat;
mod io;
mod serialize;


pub struct NewDocument
{
    pub organization: Option<String>,
    pub doc_type: Option<String>,
    pub number: Option<String>,
    pub sign_date: Option<String>,
    pub parse_time: String
}
impl NewDocument
{
    pub fn new() -> Self
    {
        Self
        {
            organization: None,
            doc_type: None,
            number: None,
            sign_date: None,
            parse_time: settings::Date::now().as_serialized()
        }
    }
}

impl From<&Packet> for NewDocument
{
    fn from(value: &Packet) -> Self 
    {
        let organization = value.get_organization().map_or( None, |o| Some(o.into_owned()));
        let date_number = value.get_document_date_number();
        let date = date_number.as_ref().and_then(|d| d.date.clone());
        let number = date_number.as_ref().and_then(|d| d.number.clone());
        Self
        {
            organization,
            doc_type: value.get_document_type(),
            number,
            sign_date: date,
            parse_time: settings::Date::now().as_serialized()
        }
    }
}