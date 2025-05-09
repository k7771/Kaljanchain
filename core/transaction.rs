// Kaljanchain Transactions - Basic Transaction Structure with Signature Validation (Rust)

use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fmt;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier, KEYPAIR_LENGTH};
use rand::rngs::OsRng;

// === Основна структура транзакції ===
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
    pub hash: String,
}

impl Transaction {
    // Створення нової транзакції
    pub fn new(sender: String, recipient: String, amount: f64, keypair: &Keypair) -> Self {
        let timestamp = Utc::now();
        let mut transaction = Transaction {
            sender: sender.clone(),
            recipient,
            amount,
            timestamp,
            signature: String::new(),
            hash: String::new(),
        };
        transaction.hash = transaction.calculate_hash();
        transaction.signature = transaction.sign(&keypair);
        transaction
    }

    // Розрахунок хеша транзакції
    pub fn calculate_hash(&self) -> String {
        let data = format!("{}{}{}{}", self.sender, self.recipient, self.amount, self.timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // Підпис транзакції
    pub fn sign(&self, keypair: &Keypair) -> String {
        let message = self.hash.as_bytes();
        let signature = keypair.sign(message);
        base64::encode(signature.to_bytes())
    }

    // Перевірка підпису транзакції
    pub fn is_valid(&self, public_key: &PublicKey) -> bool {
        let message = self.hash.as_bytes();
        match base64::decode(&self.signature) {
            Ok(signature_bytes) => {
                match Signature::from_bytes(&signature_bytes) {
                    Ok(signature) => public_key.verify(message, &signature).is_ok(),
                    Err(_) => false,
                }
            },
            Err(_) => false,
        }
    }
}

// Тестування транзакцій з підписами
fn main() {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);
    let public_key = PublicKey::from(&keypair);
    let tx = Transaction::new(
        String::from("Alice"),
        String::from("Bob"),
        10.5,
        &keypair
    );
    println!("Транзакція: {:?}", tx);
    println!("Дійсність транзакції: {}", tx.is_valid(&public_key));
}
