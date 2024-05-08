
mod contract;
mod packet;
pub use packet::Packet;
use anyhow::{Result, Context};
use bytes::Bytes;
pub use contract::Contract;
use settings::{Settings, Task};
use serde::{Serialize, Deserialize};
pub use medo_parser::DeliveryTicketPacket;
pub use medo_parser::MedoParser;
pub use medo_parser::{Ack, PacketInfo, Executor, Requisites, SenderInfo, MinistryOfJustice};

///Трейт для сериализации\десериализации данных через http
/// надо вынести его в отдельный функционал и не мешать с websocket? может его в вебсокет и отправить? тем более фючи совпадают и ящики тоже
pub trait BytesSerializer
{
    #[cfg(feature="json")]
    fn to_bytes(&self) -> Result<Bytes> where Self: Serialize
    {
        let mut bytes: Vec<u8> = Vec::new();
        serde_json::to_writer(&mut bytes, self)?;
        Ok(Bytes::from_iter(bytes))
    }
    #[cfg(feature="json")]
    fn from_bytes(body: &Bytes) -> anyhow::Result<Self> where for <'de> Self : Deserialize<'de>
    {
        let obj = serde_json::from_slice::<Self>(body).with_context(|| format!("Данный объект отличается от того который вы хотите получить"))?;
        return Ok(obj);
    }
    #[cfg(feature="flexbuffers")]
    fn to_bytes(&self) -> Result<Bytes> where Self: Serialize
    {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        let _ = self.serialize(&mut s)?;
        Ok(Bytes::copy_from_slice(s.view()))
    }
    #[cfg(feature="flexbuffers")]
    fn from_bytes(body: &Bytes) -> anyhow::Result<Self> where for <'de> Self : Deserialize<'de>
    {
        let r = flexbuffers::Reader::get_root(body.as_ref())?;
        let deserialize = Self::deserialize(r).with_context(|| "Ошибка десериализации".to_owned())?;
        Ok(deserialize)
    }
    #[cfg(feature="binary")]
    fn to_bytes(&self) -> Result<Bytes> where Self: bitcode::Encode
    {
        let encoded = bitcode::encode(self);
        Ok(Bytes::copy_from_slice(&encoded))
    }
    #[cfg(feature="binary")]
    fn from_bytes(body: &Bytes) -> anyhow::Result<Self> where for <'de> Self : bitcode::Decode<'de>
    {
        let obj = bitcode::decode::<Self>(body).with_context(|| format!("Данный объект отличается от того который вы хотите получить"))?;
        return Ok(obj);
    }
}
impl BytesSerializer for Settings{}
impl BytesSerializer for Task{}
impl BytesSerializer for Vec<Task>{}
impl BytesSerializer for u32{}
impl BytesSerializer for Contract{}
impl BytesSerializer for Packet{}
impl BytesSerializer for Vec<PacketInfo>{}
impl BytesSerializer for Vec<Packet>{}
impl BytesSerializer for PacketInfo{}