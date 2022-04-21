type Hash = [char; 32];

struct Block {
    version: i32,
    previous_block_header_hash: Hash,
    merkle_root_hash: Hash,
    time: u32,
    nbits: u32,
    nonce: u32,
}
