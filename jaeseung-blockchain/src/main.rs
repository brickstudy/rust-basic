mod block;

use block::Block;

fn main() {
    let block = Block::new_genesis();

    print!("{:?}", block);
}
