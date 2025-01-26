mod block;
mod chain;

use block::Block;

fn main() {
    let block = Block::new_genesis();

    print!("{:?}", block);
}
