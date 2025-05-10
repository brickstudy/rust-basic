use crate::{block::Block, constants::DIFFICULTY_ADJUSTMENT_INTERVAL_COUNT};

#[derive(Debug)]
pub struct Chain {
    block_chain: Vec<Block>,
}

impl Chain {
    pub fn new() -> Result<Chain, String> {
        Block::new_genesis().map(|b| Chain {
            block_chain: vec![b],
        })
    }

    pub fn get_length(&self) -> u32 {
        self.block_chain.len() as u32
    }

    pub fn get_lastest_block(&self) -> Option<&Block> {
        self.block_chain.last()
    }

    pub fn add_block(&mut self, data: &Vec<String>) -> Result<&Chain, String> {
        let previous_block = self
            .get_lastest_block()
            .ok_or("previous_block not exist.")?;
        let adjustment_block = self.get_adjustment_block()?;

        let new_block = Block::new(previous_block, data, &adjustment_block)?;

        self.block_chain.push(new_block); // TODO : 안전할까?
        Ok(self)
    }

    fn get_adjustment_block(&self) -> Result<Block, String> {
        let current_length = self.get_length();

        match current_length {
            ..DIFFICULTY_ADJUSTMENT_INTERVAL_COUNT => Block::new_genesis(),
            _ => Ok(self.block_chain
                [(current_length - DIFFICULTY_ADJUSTMENT_INTERVAL_COUNT) as usize]
                .clone()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_chain() {
        let chain = Chain::new().unwrap();

        assert_eq!(chain.block_chain.len(), 1);
    }

    #[test]
    fn get_length() {
        let chain = Chain::new().unwrap();

        assert_eq!(chain.get_length(), 1);
    }

    #[test]
    fn get_lastest_block() {
        let first_block = Block::new_genesis().unwrap();
        let second_block = Block::new(&first_block, 0, &vec!["data".to_string()]).unwrap();

        let chain = Chain {
            block_chain: vec![first_block, second_block.clone()],
        };

        assert_eq!(*chain.get_lastest_block().unwrap(), second_block);
    }

    #[test]
    fn add_block_to_empty_chain() {
        let mut empty_chain = Chain {
            block_chain: Vec::new(),
        };
        let result = empty_chain.add_block(&vec!["data".to_string()]);

        assert_eq!(result.is_err(), true);
        assert_eq!(empty_chain.block_chain.len(), 0);
    }

    #[test]
    fn add_block() {
        let mut chain = Chain::new().unwrap();
        let added_chain = chain.add_block(&vec!["data".to_string()]).unwrap();

        assert_eq!(added_chain.block_chain.len(), 2);
    }
}
