use anyhow::Result;
use std::{
    io::{BufRead, BufReader},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
};

use chainparams::TESTNET_PARAMS;
use util::find_seed_node;

use crate::protocol::{P2PProtocol, P2PProtocolSerDeTrait};

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

        Self { seed_ips, port }
    }

    pub fn new_testnet() -> Self {
        Self::new(
            &[
                "testnet-seed.bitcoin.jonasschnelli.ch.",
                "seed.tbtc.petertodd.org.",
                "seed.testnet.bitcoin.sprovoost.nl.",
                "testnet-seed.bluematt.me.",
            ],
            TESTNET_PARAMS.port,
        )
    }

    pub fn run(self) -> Result<()> {
        // self.sync();
        let sock = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(sock)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    Self::handle_connection(stream);
                }
                Err(e) => eprintln!("Peer Connection Error: {}", e),
            }
        }

        Ok(())
    }

    pub fn handle_connection(stream: TcpStream) {
        let mut reader = BufReader::new(&stream);
        loop {
            loop {
                let buffer = reader.fill_buf().expect("failed to read from the socket");
                let m = P2PProtocol::serialize(buffer);
                println!("read: {:?}", m);
            }
        }
    }

    pub fn sync(self) -> Result<()> {
        let mut stream: Option<TcpStream> = None;
        for ip in self.seed_ips {
            let sock = SocketAddr::new(ip, self.port);
            if let Ok(s) = TcpStream::connect(sock) {
                stream = Some(s);
                break;
            }
        }

        let stream = stream.expect("Can't connect to other peer");

        println!("{:?}", stream);

        Ok(())
    }
}
