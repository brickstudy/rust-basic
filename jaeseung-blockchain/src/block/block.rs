use std::time::{SystemTime, UNIX_EPOCH};

use sha256::digest;

use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

#[derive(Debug, PartialEq, Clone)] // TODO : Copy 와 Clone 의 차이점?
pub struct Block {
    header: BlockHeader,
    data: Vec<String>,
}

impl Block {
    const GENESIS_DATA: &str = "This is GENESIS block";

    pub fn new_genesis() -> Result<Block, String> {
        let data = vec![Self::GENESIS_DATA.to_string()];

        Ok(Block {
            header: BlockHeader::make(0, &["0"; 64].join(""), 0, &data)?,
            data: data,
        })
    }

    pub fn new(previous_block: &Block, nonce: i32, data: &Vec<String>) -> Result<Block, String> {
        let p_header = &previous_block.header;

        Ok(Block {
            header: BlockHeader::make(p_header.height + 1, &p_header.hash, nonce, data)?,
            data: data.clone(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BlockHeader {
    version: String,
    height: i32, // 블록의 높이. 블록 체인에 연결된 블록의 수
    timestamp: u64,
    hash: String, // 생성일시, 버전, bits, 머클루트, previousHash, nonce 의 조합을 해싱한 값
    previous_hash: Option<String>, // 이전 블록의 해싱 값
    merkle_root: String, // https://brunch.co.kr/@gapcha/263
    nonce: i32,
    difficulty: i32,
}

impl BlockHeader {
    const VERSION: &str = "1.0.0";
    const DIFFICULTY: i32 = 0;

    fn make(
        height: i32,
        previous_hash: &String,
        nonce: i32,
        data: &Vec<String>,
    ) -> Result<BlockHeader, String> {
        let version = Self::VERSION.to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let difficulty = Self::DIFFICULTY;
        let merkle_root = Self::make_merkle_root(data).ok_or("Merkle Tree Parsing Failed")?;

        Ok(BlockHeader {
            version,
            height,
            timestamp,
            hash: Self::make_hash(timestamp, &merkle_root, previous_hash, nonce),
            previous_hash: Some(previous_hash.clone()),
            merkle_root,
            nonce,
            difficulty,
        })
    }

    // TODO : data 가 빈 배열일 때 오류남.
    fn make_merkle_root(data: &Vec<String>) -> Option<String> {
        let mut merkle_tree: MerkleTree<Sha256> = MerkleTree::new();
        let mut leaves: Vec<[u8; 32]> = data.iter().map(|x| Sha256::hash(x.as_bytes())).collect();
        merkle_tree.append(&mut leaves).commit();

        merkle_tree.root_hex()
    }

    fn make_hash(
        timestamp: u64,
        merkle_root: &String,
        previous_hash: &String,
        nonce: i32,
    ) -> String {
        let target = format!(
            "{}{}{}{}",
            timestamp.to_string(),
            merkle_root,
            previous_hash,
            nonce.to_string()
        );
        digest(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_genesis() {
        let actual = Block::new_genesis().unwrap();

        let data = vec![Block::GENESIS_DATA.to_string()];
        let expected = Block {
            header: BlockHeader::make(0, &["0"; 64].join(""), 0, &data).unwrap(),
            data,
        };
        assert_eq!(actual, expected);
    }
}
