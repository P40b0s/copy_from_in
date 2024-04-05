
use anyhow::{Result, Context};
use bytes::Bytes;
use settings::{Settings, Task};
use serde::{Serialize, Deserialize};

pub trait BytesSerializer
{
    fn to_bytes(&self) -> Result<Bytes> where Self: Serialize
    {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        let _ = self.serialize(&mut s)?;
        Ok(Bytes::copy_from_slice(s.view()))
    }
    fn from_bytes(body: &Bytes) -> anyhow::Result<Self> where for <'de> Self : Deserialize<'de>
    {
        let r = flexbuffers::Reader::get_root(body.as_ref())?;
        let deserialize = Self::deserialize(r).with_context(|| "Ошибка десериализации".to_owned())?;
        Ok(deserialize)
    }
}
impl BytesSerializer for Settings{}
impl BytesSerializer for Task{}
impl BytesSerializer for Vec<Task>{}
impl BytesSerializer for u32{}