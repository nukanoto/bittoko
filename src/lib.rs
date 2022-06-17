use std::net::IpAddr;

use util::find_seed_node;

pub mod block;
pub mod chainparams;
pub mod protocol;
pub mod util;

type DnsSeeds<'a> = &'a [&'a str];

pub struct Bittoko {
    seed_ips: Vec<IpAddr>,
    port: u16,
}

impl Bittoko {
    pub fn new(dns_seeds: DnsSeeds, port: u16) -> Self {
        let seed_ips = find_seed_node(dns_seeds);

        Self {
            seed_ips,
            port,
        }
    }

    pub fn new_testnet() -> Self {
        Self::new(
            &[
                "testnet-seed.bitcoin.jonasschnelli.ch.",
                "seed.tbtc.petertodd.org.",
                "seed.testnet.bitcoin.sprovoost.nl.",
                "testnet-seed.bluematt.me.",
            ],
            18333,
        )
    }

    pub fn sync(self) {}
}
