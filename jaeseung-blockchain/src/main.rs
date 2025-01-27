mod block;
mod chain;
mod constants;

use block::Block;
use chain::Chain;

fn main() {
    let mut chain = Chain::new().unwrap();

    for x in 1..30000 {
        chain.add_block(&vec![format!("data-{x}").to_string()]);
    }

    print!("{:#?}", chain);
}
