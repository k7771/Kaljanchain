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
	Address string
	Active  bool
	LastSeen time.Time
}

type Blockchain struct {
	Blocks      []Block
	Nodes       map[string]Node
	Mutex       sync.Mutex
	Difficulty  int
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
		Difficulty: 4,
	}
	return bc
}

// === Основний код запуску вузла ===
func main() {
	bc := NewBlockchain()
	fmt.Println("Kaljanchain Node запущено...")
	fmt.Printf("Genesis Block: %v\n", bc.Blocks[0])
	
	// Тестове додавання блоку
	newBlock := bc.AddBlock("Test Transaction", "Miner-123")
	fmt.Printf("New Block: %v\n", newBlock)
}
