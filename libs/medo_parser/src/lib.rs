#[cfg(feature = "all")]
extern crate thiserror;
#[cfg(feature = "all")]
mod parser;
#[cfg(feature = "all")]
mod error;
#[cfg(test)]
mod tests;
#[cfg(feature = "all")]
mod io;
#[cfg(feature = "all")]
mod traits;
#[cfg(feature = "all")]
mod modules;
#[cfg(feature = "all")]
mod helpers;
#[cfg(any(feature = "model", feature = "all"))]
mod delivery_tickcket_packet;
#[cfg(any(feature = "model", feature = "all"))]
mod medo_model;
#[cfg(feature = "all")]
use helpers::*;
#[cfg(feature = "all")]
mod converters;
#[cfg(feature = "all")]
use modules::*;
#[cfg(feature = "all")]
mod xml;
#[cfg(feature = "all")]
use xml::*;
#[cfg(feature = "all")]
use traits::Uid;
#[cfg(feature = "all")]
use io::*;
#[cfg(feature = "all")]
mod packet;
#[cfg(feature = "all")]
use packet::Packet;
#[cfg(feature = "all")]
use error::MedoParserError;
#[cfg(feature = "all")]
mod deserializers;
#[cfg(feature = "all")]
use deserializers::guid_deserializer;
#[cfg(any(feature = "model", feature = "all"))]
pub use delivery_tickcket_packet::DeliveryTicketPacket;
#[cfg(feature = "all")]
pub use parser::MedoParser;
#[cfg(any(feature = "model", feature = "all"))]
pub use medo_model::{Ack, PacketInfo, Executor, Requisites, SenderInfo, MinistryOfJustice};