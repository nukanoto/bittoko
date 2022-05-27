use std::{collections::HashSet, net::IpAddr};

use dns_lookup::lookup_host;

mod block;

type DnsSeeds<'a> = &'a [&'a str];

pub struct Bittoko<'a> {
    dns_seeds: DnsSeeds<'a>,
}

impl<'a> Bittoko<'a> {
    pub fn new(dns_seeds: DnsSeeds<'a>) -> Self {
        Self { dns_seeds }
    }

    pub fn new_testnet() -> Self {
        Self::new(&[
            "testnet-seed.bitcoin.jonasschnelli.ch.",
            "seed.tbtc.petertodd.org.",
            "seed.testnet.bitcoin.sprovoost.nl.",
            "testnet-seed.bluematt.me.",
        ])
    }

    pub fn find_seed_node(self) -> Vec<IpAddr> {
        let mut node_ips = Vec::new();
        for dns_seed in self.dns_seeds {
            let ips = lookup_host(dns_seed).expect("Failed to lookup DNS seed");
            node_ips.append(&mut ips.to_vec());
        }
        node_ips
    }
}
