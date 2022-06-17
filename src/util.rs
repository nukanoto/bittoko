use dns_lookup::lookup_host;
use std::net::IpAddr;

use crate::DnsSeeds;

pub fn find_seed_node(dns_seeds: DnsSeeds) -> Vec<IpAddr> {
    let mut seed_ips = Vec::new();
    for dns_seed in dns_seeds {
        let ips = lookup_host(dns_seed).expect("Failed to lookup DNS seed");
        seed_ips.append(&mut ips.to_vec());
    }
    seed_ips
}
