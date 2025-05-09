package main

import (
	"fmt"
	"log"
	"net/http"
	"time"
	"encoding/json"
	"github.com/gorilla/websocket"
)

// NodeStatus представляє структуру для передачі стану вузлів.
type NodeStatus struct {
	Address    string `json:"address"`
	Status     string `json:"status"`
	LastCheck  string `json:"last_check"`
	ErrorCount int    `json:"error_count"`
}

// Перемикач для WebSocket.
var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		return true
	},
}

// Список вузлів для моніторингу.
var nodes = []NodeStatus{
	{"127.0.0.1:8080", "unknown", "", 0},
	{"192.168.1.100:8080", "unknown", "", 0},
	{"example.com:8080", "unknown", "", 0},
}

// Обробник WebSocket з'єднання.
func nodeStatusHandler(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("Помилка при створенні WebSocket з'єднання:", err)
		return
	}
	defer conn.Close()

	for {
		for i := range nodes {
			nodes[i].LastCheck = time.Now().Format("2006-01-02 15:04:05")
			nodes[i].Status = checkNode(nodes[i].Address)
			jsonData, _ := json.Marshal(nodes)
			if err := conn.WriteMessage(websocket.TextMessage, jsonData); err != nil {
				log.Println("Помилка при відправці даних:", err)
				return
			}
		}
		time.Sleep(5 * time.Second)
	}
}

// Функція перевірки стану вузла.
func checkNode(address string) string {
	_, err := http.Get("http://" + address)
	if err != nil {
		return "недоступний"
	}
	return "доступний"
}

// Запуск веб-сервера.
func main() {
	http.HandleFunc("/ws", nodeStatusHandler)
	log.Println("Сервер запущено на http://localhost:8081")
	log.Fatal(http.ListenAndServe(":8081", nil))
}
