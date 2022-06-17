use crate::protocol::header::StartString;

#[derive(Debug)]
pub struct ChainParams {
    pub start_string: StartString,
    pub port: u16,
}

pub const MAINNET_PARAMS: ChainParams = ChainParams {
    start_string: 0xf9beb4d9,
    port: 8333
};

pub const TESTNET_PARAMS: ChainParams = ChainParams {
    start_string: 0xf9beb4d9,
    port: 18333
};

pub const REGTEST_PARAMS: ChainParams = ChainParams {
    start_string: 0xfabfb5da,
    port: 18444
};

