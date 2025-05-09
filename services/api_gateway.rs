// Kaljanchain API - Enhanced HTTP API with Node Status and Statistics (Rust)

use std::sync::{Arc, Mutex}; use kaljanchain_core::{Blockchain, Block}; use kaljanchain_transactions::Transaction; use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier}; use rand::rngs::OsRng; use warp::Filter; use serde::{Serialize, Deserialize}; use std::collections::HashMap; use base64;

// === Основна структура API запитів === #[derive(Serialize, Deserialize, Debug, Clone)] struct TransactionRequest { sender: String, recipient: String, amount: f64, private_key: String, }

// === Основна структура API відповідей === #[derive(Serialize, Deserialize, Debug, Clone)] struct ApiResponse { status: String, message: String, }

// === Структура для історії транзакцій === #[derive(Serialize, Deserialize, Debug, Clone)] struct TransactionHistory { transactions: Vec<Transaction>, }

// === Структура для детальної інформації про блок === #[derive(Serialize, Deserialize, Debug, Clone)] struct BlockDetails { block: Block, transactions: Vec<Transaction>, }

// === Структура для статистики вузла === #[derive(Serialize, Deserialize, Debug, Clone)] struct NodeStats { block_count: usize, transaction_count: usize, mempool_size: usize, active_peers: usize, total_balance: f64, }

// Ініціалізація HTTP API async fn start_api(blockchain: Arc<Mutex<Blockchain>>, mempool: Arc<Mutex<Vec<Transaction>>>, balances: Arc<Mutex<HashMap<String, f64>>>, peers: Arc<Mutex<HashMap<String, String>>>) { let blockchain_filter = warp::any().map(move || blockchain.clone()); let mempool_filter = warp::any().map(move || mempool.clone()); let balances_filter = warp::any().map(move || balances.clone()); let peers_filter = warp::any().map(move || peers.clone());

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

// Ендпоінт для перегляду статистики вузла
let node_stats = warp::path("stats")
    .and(warp::get())
    .and(blockchain_filter.clone())
    .and(mempool_filter.clone())
    .and(balances_filter.clone())
    .and(peers_filter.clone())
    .map(|blockchain: Arc<Mutex<Blockchain>>, mempool: Arc<Mutex<Vec<Transaction>>>, balances: Arc<Mutex<HashMap<String, f64>>>, peers: Arc<Mutex<HashMap<String, String>>>| {
        let blockchain = blockchain.lock().unwrap();
        let mempool = mempool.lock().unwrap();
        let balances = balances.lock().unwrap();
        let peers = peers.lock().unwrap();
        let total_balance: f64 = balances.values().sum();
        let transaction_count: usize = blockchain.blocks.iter().map(|b| serde_json::from_str::<Vec<Transaction>>(&b.data).unwrap_or(vec![]).len()).sum();
        warp::reply::json(&NodeStats {
            block_count: blockchain.blocks.len(),
            transaction_count,
            mempool_size: mempool.len(),
            active_peers: peers.len(),
            total_balance,
        })
    });

// Запуск сервера
let routes = blockchain_status.or(check_balance).or(create_transaction).or(node_stats);
warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

}

// Тестування API #[tokio::main] async fn main() { let blockchain = Arc::new(Mutex::new(Blockchain::new(4))); let mempool = Arc::new(Mutex::new(vec![])); let balances = Arc::new(Mutex::new(HashMap::new())); let peers = Arc::new(Mutex::new(HashMap::new())); println!("Kaljanchain API запущено на http://127.0.0.1:8080"); start_api(blockchain, mempool, balances, peers).await; }

