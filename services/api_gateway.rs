// Kaljanchain API - Enhanced HTTP API with Block Details and Node Status (Rust)

use std::sync::{Arc, Mutex}; use kaljanchain_core::{Blockchain, Block}; use kaljanchain_transactions::Transaction; use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier}; use rand::rngs::OsRng; use warp::Filter; use serde::{Serialize, Deserialize}; use std::collections::HashMap; use base64;

// === Основна структура API запитів === #[derive(Serialize, Deserialize, Debug, Clone)] struct TransactionRequest { sender: String, recipient: String, amount: f64, private_key: String, }

// === Основна структура API відповідей === #[derive(Serialize, Deserialize, Debug, Clone)] struct ApiResponse { status: String, message: String, }

// === Структура для історії транзакцій === #[derive(Serialize, Deserialize, Debug, Clone)] struct TransactionHistory { transactions: Vec<Transaction>, }

// === Структура для детальної інформації про блок === #[derive(Serialize, Deserialize, Debug, Clone)] struct BlockDetails { block: Block, transactions: Vec<Transaction>, }

// Ініціалізація HTTP API async fn start_api(blockchain: Arc<Mutex<Blockchain>>, mempool: Arc<Mutex<Vec<Transaction>>>, balances: Arc<Mutex<HashMap<String, f64>>>) { let blockchain_filter = warp::any().map(move || blockchain.clone()); let mempool_filter = warp::any().map(move || mempool.clone()); let balances_filter = warp::any().map(move || balances.clone());

// Ендпоінт для перевірки стану блокчейну
let blockchain_status = warp::path("status")
    .and(warp::get())
    .and(blockchain_filter.clone())
    .map(|blockchain: Arc<Mutex<Blockchain>>| {
        let blockchain = blockchain.lock().unwrap();
        warp::reply::json(&blockchain.blocks)
    });

// Ендпоінт для перевірки балансу
let check_balance = warp::path("balance")
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>())
    .and(balances_filter.clone())
    .map(|params: HashMap<String, String>, balances: Arc<Mutex<HashMap<String, f64>>>| {
        let address = params.get("address").unwrap_or(&String::from("unknown")).clone();
        let balance = *balances.lock().unwrap().get(&address).unwrap_or(&0.0);
        warp::reply::json(&ApiResponse {
            status: String::from("success"),
            message: format!("Баланс {}: {}", address, balance),
        })
    });

// Ендпоінт для створення транзакцій
let create_transaction = warp::path("transaction")
    .and(warp::post())
    .and(warp::body::json())
    .and(mempool_filter.clone())
    .and(balances_filter.clone())
    .map(|req: TransactionRequest, mempool: Arc<Mutex<Vec<Transaction>>>, balances: Arc<Mutex<HashMap<String, f64>>>| {
        let sender_balance = *balances.lock().unwrap().get(&req.sender).unwrap_or(&0.0);
        if sender_balance < req.amount {
            return warp::reply::json(&ApiResponse {
                status: String::from("error"),
                message: format!("Недостатній баланс: {}", sender_balance),
            });
        }

        let keypair_bytes = base64::decode(req.private_key).unwrap();
        let keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
        let tx = Transaction::new(req.sender.clone(), req.recipient.clone(), req.amount, &keypair);
        mempool.lock().unwrap().push(tx.clone());

        warp::reply::json(&ApiResponse {
            status: String::from("success"),
            message: format!("Транзакція додана: {} -> {}: {}", tx.sender, tx.recipient, tx.amount),
        })
    });

// Ендпоінт для перегляду історії транзакцій
let transaction_history = warp::path("history")
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>())
    .and(blockchain_filter.clone())
    .map(|params: HashMap<String, String>, blockchain: Arc<Mutex<Blockchain>>| {
        let address = params.get("address").unwrap_or(&String::from("unknown")).clone();
        let blockchain = blockchain.lock().unwrap();
        let mut history = vec![];
        for block in &blockchain.blocks {
            let transactions: Vec<Transaction> = serde_json::from_str(&block.data).unwrap_or(vec![]);
            for tx in transactions {
                if tx.sender == address || tx.recipient == address {
                    history.push(tx);
                }
            }
        }
        warp::reply::json(&TransactionHistory { transactions: history })
    });

// Ендпоінт для перегляду деталей блоку
let block_details = warp::path("block")
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>())
    .and(blockchain_filter.clone())
    .map(|params: HashMap<String, String>, blockchain: Arc<Mutex<Blockchain>>| {
        let index = params.get("index").and_then(|i| i.parse::<usize>().ok()).unwrap_or(0);
        let blockchain = blockchain.lock().unwrap();
        if let Some(block) = blockchain.blocks.get(index) {
            let transactions: Vec<Transaction> = serde_json::from_str(&block.data).unwrap_or(vec![]);
            warp::reply::json(&BlockDetails { block: block.clone(), transactions })
        } else {
            warp::reply::json(&ApiResponse {
                status: String::from("error"),
                message: String::from("Блок не знайдено"),
            })
        }
    });

// Запуск сервера
let routes = blockchain_status.or(check_balance).or(create_transaction).or(transaction_history).or(block_details);
warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

}

// Тестування API #[tokio::main] async fn main() { let blockchain = Arc::new(Mutex::new(Blockchain::new(4))); let mempool = Arc::new(Mutex::new(vec![])); let balances = Arc::new(Mutex::new(HashMap::new())); println!("Kaljanchain API запущено на http://127.0.0.1:8080"); start_api(blockchain, mempool, balances).await; }
