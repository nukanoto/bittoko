use anyhow::Result;
use bittoko::Bittoko;

fn main() -> Result<()> {
    let b = Bittoko::new_testnet();
    b.run().unwrap();
    Ok(())
}
