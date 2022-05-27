use bittoko::Bittoko;

fn main() {
    let btk = Bittoko::new_testnet();
    println!("{:?}", btk.find_seed_node());
}
