mod block;

use block::Block;
use rs_merkle::{algorithms::Sha256, MerkleTree};

fn main() {
    let block = Block::new_genesis(0, 0, &vec![String::from("asdasd")]);

    print!("{:?}", block);
}
