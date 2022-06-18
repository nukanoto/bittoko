use self::{header::P2PProtocolHeaders, message::Message};
use anyhow::Result;

pub mod header;
pub mod message;
pub mod verack;

pub struct P2PProtocol<'a> {
    headers: P2PProtocolHeaders<'a>,
    message: Message,
}

pub trait P2PProtocolSerDeTrait<'a>
where
    Self: Sized,
{
    fn serialize(raw: &'a [u8]) -> Result<Self>;

    fn deserialize() -> &'a [u8];
}

impl P2PProtocol<'_> {}
