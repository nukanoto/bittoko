use self::header::P2PProtocolHeaders;

pub mod header;
pub mod message;
pub mod verack;

pub struct P2PProtocol<'a> {
    headers: P2PProtocolHeaders<'a>,
}

pub trait P2PProtocolSerDeTrait<'a> {
    fn serialize(raw: &'a [u8]) -> Self;

    fn deserialize() -> &'a [u8];
}

impl P2PProtocol<'_> {
}
