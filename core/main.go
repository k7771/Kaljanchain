// Kaljanchain Core - Decentralized Blockchain Node (Go)

package main

import (
	"fmt"
	"net"
	"os"
	"sync"
	"time"
	"crypto/sha256"
	"encoding/hex"
	"math/rand"
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
)

// === Основні структури блокчейну ===
type Block struct {
	Index        int
	Timestamp    time.Time
	Data         string
	PrevHash     string
	Hash         string
	Nonce        int
	Difficulty   int
	MinerAddress string
}

type Node struct {
	Address  string
	Active   bool
	LastSeen time.Time
	Location string
}

type Wallet struct {
	Address  string
	Balance  int
	Tokens   map[string]int
	2FA      bool
	DeviceID string
	Multisig []string
	Location string
	Context  map[string]string
}

type Blockchain struct {
	Blocks        []Block
	Nodes         map[string]Node
	Wallets       map[string]Wallet
	Mutex         sync.Mutex
	Difficulty    int
	Tokens        map[string]int
	SmartContracts map[string]string
	Logs          []string
	Alerts        []string
	Analytics     map[string]int
	ScaleFactor   int
	BannedIPs     map[string]bool
	Geolocations  map[string]string
	Contexts      map[string]map[string]string
}

// === Функції ядра ===
func (bc *Blockchain) AddBlock(data, minerAddress string) Block {
	bc.Mutex.Lock()
	defer bc.Mutex.Unlock()
	
	prevBlock := bc.Blocks[len(bc.Blocks)-1]
	newBlock := Block{
		Index:      len(bc.Blocks),
		Timestamp:  time.Now(),
		Data:       data,
		PrevHash:   prevBlock.Hash,
		Difficulty: bc.Difficulty,
		MinerAddress: minerAddress,
	}
	newBlock.Hash = bc.MineBlock(&newBlock)
	bc.Blocks = append(bc.Blocks, newBlock)
	bc.Logs = append(bc.Logs, fmt.Sprintf("Block %d mined by %s", newBlock.Index, minerAddress))
	bc.Analytics[minerAddress]++
	bc.ScaleFactor++
	return newBlock
}

func (bc *Blockchain) MineBlock(block *Block) string {
	for {
		hash := bc.CalculateHash(block)
		if hash[:block.Difficulty] == string(make([]byte, block.Difficulty)) {
			return hash
		}
		block.Nonce++
	}
}

func (bc *Blockchain) CalculateHash(block *Block) string {
	data := fmt.Sprintf("%d%s%s%s%d%d", block.Index, block.Timestamp, block.Data, block.PrevHash, block.Nonce, block.Difficulty)
	hash := sha256.Sum256([]byte(data))
	return hex.EncodeToString(hash[:])
}

// === Ініціалізація блокчейну ===
func NewBlockchain() *Blockchain {
	genesisBlock := Block{
		Index:      0,
		Timestamp:  time.Now(),
		Data:       "Genesis Block",
		PrevHash:   "",
		Hash:       "",
		Difficulty: 4,
	}
	genesisBlock.Hash = sha256.New().Sum(nil)
	bc := &Blockchain{
		Blocks:     []Block{genesisBlock},
		Nodes:      make(map[string]Node),
		Wallets:    make(map[string]Wallet),
		Difficulty: 4,
		Tokens:     make(map[string]int),
		SmartContracts: make(map[string]string),
		Logs:       []string{"Genesis Block Created"},
		Alerts:     []string{},
		Analytics:  make(map[string]int),
		ScaleFactor: 1,
		BannedIPs:  make(map[string]bool),
		Geolocations: make(map[string]string),
		Contexts:    make(map[string]map[string]string),
	}
	return bc
}

// === Основний код запуску вузла ===
func main() {
	bc := NewBlockchain()
	fmt.Println("Kaljanchain Node запущено...")
	fmt.Printf("Genesis Block: %v\n", bc.Blocks[0])
	
	// Токени
	bc.Tokens["KaljanCoin"] = 1000000
	fmt.Println("Токен KaljanCoin створено з початковим обсягом 1,000,000")

	// Смарт-контракти
	bc.SmartContracts["SampleContract"] = "function transfer() { return 'Success'; }"
	fmt.Println("Смарт-контракт SampleContract створено")

	// Додавання гаманця
	bc.Wallets["Wallet-123"] = Wallet{Address: "Wallet-123", Balance: 1000, Tokens: map[string]int{"KaljanCoin": 1000}, 2FA: true, DeviceID: "Device-123", Multisig: []string{"User1", "User2"}, Location: "Ukraine", Context: map[string]string{"device_type": "mobile", "ip": "192.168.1.100"}}
	fmt.Println("Гаманець Wallet-123 створено")

	// Додавання геолокації
	bc.Geolocations["Wallet-123"] = "Ukraine"

	// Запуск HTTP API
	http.HandleFunc("/status", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(bc)
	})
	http.HandleFunc("/mine", func(w http.ResponseWriter, r *http.Request) {
		miner := r.URL.Query().Get("miner")
		newBlock := bc.AddBlock("Mining Reward", miner)
		json.NewEncoder(w).Encode(newBlock)
	})
	http.HandleFunc("/add-node", func(w http.ResponseWriter, r *http.Request) {
		address := r.URL.Query().Get("address")
		location := r.URL.Query().Get("location")
		bc.ConnectNode(address, location)
		w.Write([]byte("Node added: " + address))
	})
	http.HandleFunc("/wallets", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(bc.Wallets)
	})
	http.HandleFunc("/ban-ip", func(w http.ResponseWriter, r *http.Request) {
		ip := r.URL.Query().Get("ip")
		bc.BannedIPs[ip] = true
		w.Write([]byte("IP Banned: " + ip))
	})
	log.Fatal(http.ListenAndServe(":8080", nil))
}

// === Додаткові модулі ===

// Майнінг модуль
func (bc *Blockchain) StartMining(minerAddress string) {
	for {
		bc.AddBlock("Mining Reward", minerAddress)
		time.Sleep(time.Second * 5)
	}
}

// Модуль смарт-контрактів
func (bc *Blockchain) ExecuteSmartContract(data string) Block {
	return bc.AddBlock(data, "SmartContract-Execution")
}

// Модуль безпеки (2FA, U2F, WebAuthn)
func SecureAccess(node *Node, key string) bool {
	// Placeholder for 2FA, U2F, WebAuthn implementation
	return key == "secure-access"
}

// Модуль цифрових активів
func (bc *Blockchain) CreateDigitalAsset(assetName string) Block {
	return bc.AddBlock(assetName, "Asset-Creation")
}

// Мережевий модуль (P2P)
func (bc *Blockchain) ConnectNode(address, location string) {
	bc.Mutex.Lock()
	defer bc.Mutex.Unlock()
	bc.Nodes[address] = Node{Address: address, Active: true, LastSeen: time.Now(), Location: location}
}

// Збереження стану блокчейну
func saveBlockchain(bc *Blockchain) {
	data, err := json.MarshalIndent(bc, "", "  ")
	if err != nil {
		log.Fatalf("Помилка збереження стану блокчейну: %v", err)
	}
	err = ioutil.WriteFile("blockchain_state.json", data, 0644)
	if err != nil {
		log.Fatalf("Помилка запису стану блокчейну у файл: %v", err)
	}
}
