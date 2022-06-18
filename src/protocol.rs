use self::{header::P2PProtocolHeaders, message::Message};
use anyhow::Result;

pub mod header;
pub mod message;
pub mod verack;

#[derive(Debug)]
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

impl<'a> P2PProtocolSerDeTrait<'a> for P2PProtocol<'a> {
    fn serialize(raw: &'a [u8]) -> Result<Self> {
        let headers = P2PProtocolHeaders::serialize(raw)?;
        let message = Message::serialize(raw, headers.command_name)?;

        Ok(P2PProtocol { headers, message })
    }

    fn deserialize() -> &'a [u8] {
        todo!()
    }
}
