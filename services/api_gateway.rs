// Kaljanchain API - Basic HTTP API for Node Interaction (Rust)

use std::sync::{Arc, Mutex}; use kaljanchain_core::{Blockchain, Block}; use kaljanchain_transactions::Transaction; use ed25519_dalek::{Keypair, PublicKey}; use rand::rngs::OsRng; use warp::Filter; use serde::{Serialize, Deserialize};

// === Основна структура API запитів === #[derive(Serialize, Deserialize, Debug, Clone)] struct TransactionRequest { sender: String, recipient: String, amount: f64, private_key: String, }

// === Основна структура API відповідей === #[derive(Serialize, Deserialize, Debug, Clone)] struct ApiResponse { status: String, message: String, }

// Ініціалізація HTTP API async fn start_api(blockchain: Arc<Mutex<Blockchain>>, mempool: Arc<Mutex<Vec<Transaction>>>) { let blockchain_filter = warp::any().map(move || blockchain.clone()); let mempool_filter = warp::any().map(move || mempool.clone());

// Ендпоінт для перевірки стану блокчейну
let blockchain_status = warp::path("status")
    .and(warp::get())
    .and(blockchain_filter.clone())
    .map(|blockchain: Arc<Mutex<Blockchain>>| {
        let blockchain = blockchain.lock().unwrap();
        warp::reply::json(&blockchain.blocks)
    });

// Ендпоінт для створення транзакцій
let create_transaction = warp::path("transaction")
    .and(warp::post())
    .and(warp::body::json())
    .and(mempool_filter.clone())
    .map(|req: TransactionRequest, mempool: Arc<Mutex<Vec<Transaction>>>| {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        let tx = Transaction::new(req.sender, req.recipient, req.amount, &keypair);
        mempool.lock().unwrap().push(tx.clone());
        warp::reply::json(&ApiResponse {
            status: String::from("success"),
            message: format!("Транзакція додана: {} -> {}: {}", tx.sender, tx.recipient, tx.amount),
        })
    });

// Запуск сервера
let routes = blockchain_status.or(create_transaction);
warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

}

// Тестування API #[tokio::main] async fn main() { let blockchain = Arc::new(Mutex::new(Blockchain::new(4))); let mempool = Arc::new(Mutex::new(vec![])); println!("Kaljanchain API запущено на http://127.0.0.1:8080"); start_api(blockchain, mempool).await; }

