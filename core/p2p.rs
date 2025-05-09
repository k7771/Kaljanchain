// Kaljanchain P2P Network - Node Communication, Sync and Auto-connect (Rust)

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use kaljanchain_core::{Blockchain, Block};
use kaljanchain_transactions::Transaction;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
use std::thread::sleep;

// === Основна структура повідомлення ===
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Block(Block),
    Transaction(Transaction),
    SyncRequest,
    SyncResponse(Vec<Block>),
    MempoolRequest,
    MempoolResponse(Vec<Transaction>),
    BalanceRequest(String),
    BalanceResponse(String, f64),
    TransactionConfirmation(Transaction),
    DoubleSpendAlert(Transaction),
    BalanceRollback(String, f64),
    NodeCheck(String),
    NodeCheckResponse(String, bool),
    NodeRemove(String),
}

// === Основна структура вузла ===
pub struct Node {
    pub address: String,
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub peers: Arc<Mutex<HashMap<String, TcpStream>>>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
    pub balances: Arc<Mutex<HashMap<String, f64>>>,
    pub mining_reward: f64,
    pub max_block_size: usize,
    pub last_sync: Arc<Mutex<Instant>>, // час останньої синхронізації
}

impl Node {
    // Ініціалізація вузла
    pub fn new(address: String, blockchain: Arc<Mutex<Blockchain>>, mining_reward: f64, max_block_size: usize) -> Self {
        Node {
            address,
            blockchain,
            peers: Arc::new(Mutex::new(HashMap::new())),
            mempool: Arc::new(Mutex::new(vec![])),
            balances: Arc::new(Mutex::new(HashMap::new())),
            mining_reward,
            max_block_size,
            last_sync: Arc::new(Mutex::new(Instant::now())),
        }
    }

    // Запуск вузла
    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).expect("Не вдалося запустити вузол");
        println!("Вузол запущено на {}", self.address);

        // Старт автоматичної перевірки вузлів
        let self_clone = self.clone();
        thread::spawn(move || {
            self_clone.start_node_health_check();
        });

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let peer_address = stream.peer_addr().unwrap().to_string();
                    println!("Підключено нового вузла: {}", peer_address);
                    self.add_peer(peer_address.clone(), stream);
                    self.sync_blocks(&peer_address);
                },
                Err(e) => println!("Помилка при підключенні клієнта: {}", e),
            }
        }
    }

    // Додавання нового вузла в список піров
    pub fn add_peer(&self, address: String, stream: TcpStream) {
        let mut peers = self.peers.lock().unwrap();
        peers.insert(address, stream);
        println!("Вузол {} додано до списку піров.", address);
    }

    // Синхронізація блоків з новим вузлом
    pub fn sync_blocks(&self, address: &str) {
        println!("Синхронізація блоків з вузлом: {}", address);
        let blockchain = self.blockchain.lock().unwrap();
        let message = Message::SyncResponse(blockchain.blocks.clone());
        let serialized = serde_json::to_string(&message).unwrap();
        self.send_message(address, &serialized);
        let mut last_sync = self.last_sync.lock().unwrap();
        *last_sync = Instant::now(); // оновлюємо час останньої синхронізації
    }

    // Майнінг блоку з транзакціями
    pub fn mine_block(&self, miner_address: &str, keypair: &Keypair) {
        let mut mempool = self.mempool.lock().unwrap();
        if mempool.is_empty() {
            println!("Mempool порожній. Немає транзакцій для майнінгу.");
            return;
        }

        // Вибір транзакцій для блоку
        let valid_transactions: Vec<Transaction> = mempool.iter()
            .filter(|tx| self.has_sufficient_balance(tx))
            .cloned()
            .collect();

        if valid_transactions.is_empty() {
            println!("Немає дійсних транзакцій для майнінгу.");
            return;
        }

        // Додавання винагороди за майнінг
        let reward_tx = Transaction::new(
            String::from("Kaljanchain"),
            String::from(miner_address),
            self.mining_reward,
            keypair
        );
        let mut all_transactions = valid_transactions.clone();
        all_transactions.push(reward_tx.clone());

        // Додавання блоку
        let mut blockchain = self.blockchain.lock().unwrap();
        let block_data = serde_json::to_string(&all_transactions).unwrap();
        let block = Block::new(
            blockchain.blocks.len() as u64,
            blockchain.blocks.last().unwrap().hash.clone(),
            block_data
        );
        blockchain.blocks.push(block.clone());

        // Оновлення балансу майнера
        self.update_balances(miner_address, self.mining_reward);

        println!("Новий блок змайнено: {}", block.hash);

        // Підтвердження транзакцій
        for tx in all_transactions {
            self.broadcast(&Message::TransactionConfirmation(tx));
        }
    }

    // Оновлення балансів після майнінгу
    pub fn update_balances(&self, miner_address: &str, reward: f64) {
        let mut balances = self.balances.lock().unwrap();
        let balance = balances.entry(miner_address.to_string()).or_insert(0.0);
        *balance += reward;
        println!("Баланс оновлено: {} -> {}", miner_address, balance);
    }

    // Перевірка балансу для транзакцій
    pub fn has_sufficient_balance(&self, tx: &Transaction) -> bool {
        let balances = self.balances.lock().unwrap();
        if let Some(balance) = balances.get(&tx.sender) {
            return *balance >= tx.amount;
        }
        false
    }
}

// Тестування автоматичного підключення вузлів, майнінгу та синхронізації
fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new(4)));
    let node = Node::new(String::from("127.0.0.1:8080"), blockchain.clone(), 50.0, 100);

    // Генерація ключів для майнінгу
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);
    node.mine_block("Miner1", &keypair);
    node.start();
}
