// Kaljanchain P2P Network - Initial Node Communication (Rust)

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// === Основна структура повідомлення ===
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub content: String,
}

// Обробка вхідних з'єднань
fn handle_client(mut stream: TcpStream, peers: Arc<Mutex<Vec<String>>>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 { break; }
                let data = String::from_utf8_lossy(&buffer[..bytes]).to_string();
                println!("Отримано повідомлення: {}", data);
                broadcast(&data, &peers);
            },
            Err(_) => {
                println!("Помилка при читанні даних від клієнта.");
                break;
            }
        }
    }
}

// Відправка повідомлень усім вузлам
fn broadcast(message: &str, peers: &Arc<Mutex<Vec<String>>>) {
    let peers = peers.lock().unwrap();
    for peer in peers.iter() {
        if let Ok(mut stream) = TcpStream::connect(peer) {
            stream.write_all(message.as_bytes()).unwrap();
        }
    }
}

// Запуск вузла
fn start_node(address: &str) {
    let listener = TcpListener::bind(address).expect("Не вдалося запустити вузол");
    println!("Вузол запущено на {}", address);
    let peers = Arc::new(Mutex::new(vec![]));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peers = Arc::clone(&peers);
                thread::spawn(move || handle_client(stream, peers));
            },
            Err(e) => println!("Помилка при підключенні клієнта: {}", e),
        }
    }
}

// Тестування P2P мережі
fn main() {
    let address = "127.0.0.1:8080";
    start_node(address);
}
