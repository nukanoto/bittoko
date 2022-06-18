use crate::protocol::P2PProtocolSerDeTrait;
use anyhow::Result;
use scroll::{Endian, Pread};

pub type StartString = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct P2PProtocolHeaders<'a> {
    pub start_string: StartString,
    pub command_name: &'a str,
    pub payload_size: u32,
    pub checksum: u32,
}

impl<'a> P2PProtocolHeaders<'a> {
    pub fn new(
        start_string: StartString,
        command_name: &'a str,
        payload_size: u32,
        checksum: u32,
    ) -> Self {
        Self {
            start_string,
            command_name,
            payload_size,
            checksum,
        }
    }
}

impl<'a> P2PProtocolSerDeTrait<'a> for P2PProtocolHeaders<'a> {
    fn serialize(raw: &'a [u8]) -> Result<Self> {
        let mut offset = 0;
        let start_string: u32 = raw.pread_with(offset, Endian::Big)?;
        offset += 4;
        let command_name: &str = raw.pread(offset)?;
        offset += 12;
        let payload_size: u32 = raw.pread(offset)?;
        offset += 4;
        let checksum: u32 = raw.pread_with(offset, Endian::Big)?;
        // offset += 4;

        Ok(Self {
            start_string,
            command_name,
            payload_size,
            checksum,
        })
    }

    fn deserialize() -> &'a [u8] {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::chainparams::MAINNET_PARAMS;

    use super::*;

    #[test]
    fn test_serialize() {
        let bin = &[
            0xf9, 0xbe, 0xb4, 0xd9, 0x76, 0x65, 0x72, 0x61, 0x63, 0x6b, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5d, 0xf6, 0xe0, 0xe2,
        ];

        let result = P2PProtocolHeaders::serialize(bin).unwrap();

        assert_eq!(
            result,
            P2PProtocolHeaders {
                start_string: MAINNET_PARAMS.start_string,
                command_name: "verack",
                payload_size: 0,
                checksum: 0x5df6e0e2,
            }
        );
    }
}
