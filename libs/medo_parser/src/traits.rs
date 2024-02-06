use std::borrow::Cow;


pub trait Uid
{
    fn get_uid(&self) -> Cow<str>;
}