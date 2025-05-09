// Kaljanchain P2P Network - Node Communication, Sync and Auto-connect (Rust)

use std::net::{TcpListener, TcpStream}; use std::io::{Read, Write}; use std::thread; use std::sync::{Arc, Mutex}; use serde::{Serialize, Deserialize}; use std::collections::HashMap; use std::time::{Duration, Instant}; use kaljanchain_core::{Blockchain, Block}; use kaljanchain_transactions::Transaction; use ed25519_dalek::{PublicKey, Signature, Verifier};

// === Основна структура повідомлення === #[derive(Serialize, Deserialize, Debug, Clone)] pub enum Message { Block(Block), Transaction(Transaction), SyncRequest, SyncResponse(Vec<Block>), }

// === Основна структура вузла === pub struct Node { pub address: String, pub blockchain: Arc<Mutex<Blockchain>>, pub peers: Arc<Mutex<HashMap<String, TcpStream>>>, pub mempool: Arc<Mutex<Vec<Transaction>>>, pub balances: Arc<Mutex<HashMap<String, f64>>>, pub mining_reward: f64, pub max_block_size: usize, pub last_sync: Arc<Mutex<Instant>>, // час останньої синхронізації }

impl Node { // Ініціалізація вузла pub fn new(address: String, blockchain: Arc<Mutex<Blockchain>>, mining_reward: f64, max_block_size: usize) -> Self { Node { address, blockchain, peers: Arc::new(Mutex::new(HashMap::new())), mempool: Arc::new(Mutex::new(vec![])), balances: Arc::new(Mutex::new(HashMap::new())), mining_reward, max_block_size, last_sync: Arc::new(Mutex::new(Instant::now())), } }

// Запуск вузла
pub fn start(&self) {
    let listener = TcpListener::bind(&self.address).expect("Не вдалося запустити вузол");
    println!("Вузол запущено на {}", self.address);

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

// Обробка вхідних повідомлень
pub fn handle_message(&self, message: Message, public_key: &PublicKey) {
    match message {
        Message::Block(block) => {
            let mut blockchain = self.blockchain.lock().unwrap();
            if blockchain.add_block(block.data.clone()) {
                println!("Новий блок додано: {}", block.hash);
            } else {
                println!("Невалідний блок: {}", block.hash);
            }
        },
        Message::Transaction(tx) => {
            if tx.is_valid(public_key) {
                println!("Отримано дійсну транзакцію: {} -> {}: {}", tx.sender, tx.recipient, tx.amount);
                self.mempool.lock().unwrap().push(tx);
            } else {
                println!("Невалідна транзакція від {}", tx.sender);
            }
        },
        Message::SyncRequest => {
            self.sync_blocks(&self.address);
        },
        Message::SyncResponse(blocks) => {
            println!("Отримано синхронізовані блоки: {}", blocks.len());
            let mut blockchain = self.blockchain.lock().unwrap();
            for block in blocks {
                if blockchain.add_block(block.data.clone()) {
                    println!("Блок синхронізовано: {}", block.hash);
                }
            }
        },
    }
}

// Майнінг нового блоку з транзакціями з mempool
pub fn mine_block(&self, miner_address: &str) {
    let mut blockchain = self.blockchain.lock().unwrap();
    let mut mempool = self.mempool.lock().unwrap();
    if !mempool.is_empty() {
        let transactions = mempool.drain(..self.max_block_size).collect::<Vec<Transaction>>();
        let mut all_transactions = transactions.clone();

        // Додаємо винагороду за майнінг
        let reward_tx = Transaction::new(
            String::from("Kaljanchain"),
            String::from(miner_address),
            self.mining_reward,
            &ed25519_dalek::Keypair::generate(&mut rand::rngs::OsRng)
        );
        all_transactions.push(reward_tx);

        let data = serde_json::to_string(&all_transactions).unwrap();
        blockchain.add_block(data);
        println!("Новий блок з транзакціями змайнено. Загальна кількість транзакцій: {}", all_transactions.len());
    } else {
        println!("Mempool порожній. Немає транзакцій для майнінгу.");
    }
}

// Перевірка автоматичного підключення до інших вузлів
pub fn auto_connect(&self, peer_addresses: Vec<String>) {
    let mut peers = self.peers.lock().unwrap();
    for address in peer_addresses {
        if !peers.contains_key(&address) {
            match TcpStream::connect(&address) {
                Ok(stream) => {
                    peers.insert(address.clone(), stream);
                    println!("Автоматичне підключення до вузла: {} успішне", address);
                }
                Err(_) => println!("Не вдалося підключитися до вузла: {}", address),
            }
        }
    }
}

// Відправка повідомлень усім вузлам
pub fn broadcast(&self, message: &Message) {
    let peers = self.peers.lock().unwrap();
    let serialized = serde_json::to_string(message).unwrap();
    for (peer_address, stream) in peers.iter() {
        if let Err(e) = stream.try_clone().unwrap().write_all(serialized.as_bytes()) {
            println!("Помилка при відправці даних на {}: {}", peer_address, e);
        }
    }
}

// Відправка повідомлення конкретному вузлу
pub fn send_message(&self, address: &str, message: &str) {
    if let Ok(mut stream) = TcpStream::connect(address) {
        if let Err(e) = stream.write_all(message.as_bytes()) {
            println!("Помилка при відправці даних на {}: {}", address, e);
        }
    }
}

}

// Тестування автоматичного підключення вузлів та синхронізації fn main() { let blockchain = Arc::new(Mutex::new(Blockchain::new(4))); let node = Node::new(String::from("127.0.0.1:8080"), blockchain.clone(), 50.0, 100); let peer_addresses = vec!["127.0.0.1:8081".to_string()]; node.auto_connect(peer_addresses); node.start(); }

