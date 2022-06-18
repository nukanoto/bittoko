use super::verack::VerAck;
use super::P2PProtocolSerDeTrait;
use anyhow::Result;

#[derive(Debug)]
pub enum Message {
    VerAck(VerAck),
}

impl<'a> Message {
    pub fn serialize(_raw: &'a [u8], message_str: &str) -> Result<Self> {
        match message_str {
            "verack" => Ok(Self::VerAck(VerAck)),
            _ => {
                panic!("unimplemented command: {}", message_str)
            }
        }
    }

    pub fn deserialize() -> &'a [u8] {
        todo!()
    }
}
