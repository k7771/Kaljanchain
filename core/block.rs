// Kaljanchain Core - Initial Block Structure (Rust)

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use std::fmt;

// === Основна структура блоку ===
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub prev_hash: String,
    pub data: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    // Створення нового блоку
    pub fn new(index: u64, prev_hash: String, data: String) -> Self {
        let timestamp = Utc::now();
        let nonce = 0;
        let mut block = Block {
            index,
            timestamp,
            prev_hash,
            data,
            hash: String::new(),
            nonce,
        };
        block.hash = block.calculate_hash();
        block
    }

    // Розрахунок хеша блоку
    pub fn calculate_hash(&self) -> String {
        let data = format!("{}{}{}{}{}{}", self.index, self.timestamp, self.prev_hash, self.data, self.nonce, self.timestamp.timestamp_nanos());
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // Перевірка валідності блоку
    pub fn is_valid(&self, prev_block: &Block) -> bool {
        self.index == prev_block.index + 1 &&
        self.prev_hash == prev_block.hash &&
        self.hash == self.calculate_hash()
    }
}

// Тестування блоку
fn main() {
    let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));
    println!("Genesis Block: {:?}", genesis_block);

    let second_block = Block::new(1, genesis_block.hash.clone(), String::from("Second Block"));
    println!("Second Block: {:?}", second_block);
}

