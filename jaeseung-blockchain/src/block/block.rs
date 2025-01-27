use std::time::{SystemTime, UNIX_EPOCH};

use sha256::digest;

use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

use to_binary::BinaryString;

use crate::constants::{
    BLOCK_GENERATION_INTERVAL_MINUTE, BLOCK_GENERATION_MILLIS, DIFFICULTY_ADJUSTMENT_INTERVAL_COUNT,
};

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
            header: BlockHeader::make(
                0,
                Self::unsafe_get_timestamp(),
                &["0"; 64].join(""),
                0,
                &data,
            )?,
            data: data,
        })
    }

    pub fn new(
        previous_block: &Block,
        data: &Vec<String>,
        adjustment_block: &Block,
    ) -> Result<Block, String> {
        let p_header = &previous_block.header;

        let new_height = p_header.height + 1;
        let new_timestamp = Self::unsafe_get_timestamp();

        let difficulty =
            Self::get_difficulty(new_height, new_timestamp, previous_block, adjustment_block);

        Ok(Block {
            header: BlockHeader::make(
                p_header.height + 1,
                new_timestamp,
                &p_header.hash,
                difficulty,
                data,
            )?,
            data: data.clone(), // TODO : clone 이 맞을까?
        })
    }

    fn get_difficulty(
        new_height: i32,
        new_timestamp: u64,
        previous_block: &Block,
        adjustment_block: &Block,
    ) -> i32 {
        match new_height {
            0..=9 => 0,
            10..=19 => 1,
            h if h % 10 != DIFFICULTY_ADJUSTMENT_INTERVAL_COUNT as i32 => {
                previous_block.header.difficulty
            }
            _ => {
                let time_taken = new_timestamp - adjustment_block.header.timestamp;
                let time_expected = u64::from(
                    DIFFICULTY_ADJUSTMENT_INTERVAL_COUNT
                        * BLOCK_GENERATION_INTERVAL_MINUTE
                        * BLOCK_GENERATION_MILLIS,
                );

                if time_taken < time_expected / 2 {
                    adjustment_block.header.difficulty + 1
                } else if time_taken > time_expected * 2 {
                    adjustment_block.header.difficulty - 1
                } else {
                    adjustment_block.header.difficulty
                }
            }
        }
    }

    fn unsafe_get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
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

    fn make(
        height: i32,
        timestamp: u64,
        previous_hash: &String,
        difficulty: i32,
        data: &Vec<String>,
    ) -> Result<BlockHeader, String> {
        let prefix_zero = "0".repeat(difficulty as usize);

        let merkle_root = Self::make_merkle_root(data).ok_or("Merkle Tree Parsing Failed")?;

        let mut nonce = 0;

        // TODO : closure 와 꼬리 재귀를 써서 할 순 없을까?
        let (nonce, hash) = loop {
            nonce += 1;

            let hash = Self::make_block_hash(
                height,
                timestamp,
                &merkle_root,
                previous_hash,
                nonce,
                difficulty,
            );

            let binary = BinaryString::from_hex(&hash)
                .map_err(|_| "hex to binary failed".to_string())?
                .to_string();

            if binary.as_str().starts_with(prefix_zero.as_str()) {
                break (nonce, hash);
            }
        };

        Ok(BlockHeader {
            version: Self::VERSION.to_string(),
            height,
            timestamp,
            hash,
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

    fn make_block_hash(
        height: i32,
        timestamp: u64,
        merkle_root: &String,
        previous_hash: &String,
        nonce: i32,
        difficulty: i32,
    ) -> String {
        let target = format!(
            "{}{}{}{}{}{}{}",
            Self::VERSION.to_string(),
            height.to_string(),
            timestamp.to_string(),
            merkle_root,
            previous_hash,
            nonce.to_string(),
            difficulty.to_string()
        );
        digest(target)
    }
}
