package netmon

import (
	"fmt"
	"net"
	"time"
	"log"
	"os"
	"net/smtp"
	"encoding/json"
	"io/ioutil"
	"sync"
)

// Конфігурація SMTP для відправки сповіщень.
type Config struct {
	SMTPServer   string `json:"smtp_server"`
	SMTPPort     string `json:"smtp_port"`
	Username     string `json:"username"`
	Password     string `json:"password"`
	FromEmail    string `json:"from_email"`
	RetryCount   int    `json:"retry_count"`
	RetryDelay   int    `json:"retry_delay"`
	BlacklistTimeout int `json:"blacklist_timeout"`
}

// Вузол блокчейну з адресою, статусом та параметрами перевірки.
type Node struct {
	Address     string
	Status      bool
	ErrorCount  int
	MaxErrors   int
	AlertEmail  string
	lastAlert   time.Time
	alertLock   sync.Mutex
	blacklisted bool
	blacklistTime time.Time
}

// Завантаження конфігурації SMTP з JSON-файлу.
func LoadConfig(filename string) (*Config, error) {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}
	var config Config
	err = json.Unmarshal(data, &config)
	if err != nil {
		return nil, err
	}
	return &config, nil
}

// Перевірка доступності вузла та оновлення статусу.
func (n *Node) Check(config *Config) {
	if n.blacklisted && time.Since(n.blacklistTime) < time.Duration(config.BlacklistTimeout)*time.Minute {
		log.Printf("[ПОПЕРЕДЖЕННЯ] Вузол %s знаходиться в чорному списку.", n.Address)
		return
	}

	conn, err := net.DialTimeout("tcp", n.Address, 5*time.Second)
	if err != nil {
		n.Status = false
		n.ErrorCount++
		n.logError(err)
		if n.ErrorCount >= n.MaxErrors {
			n.sendAlert(config)
			n.ErrorCount = 0 // Скидання лічильника після сповіщення
			n.blacklistNode()
		}
	} else {
		n.Status = true
		n.ErrorCount = 0
		conn.Close()
		n.logInfo()
	}
}

// Логування помилок недоступності вузла.
func (n *Node) logError(err error) {
	logFile, _ := os.OpenFile("network_errors.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	defer logFile.Close()
	logger := log.New(logFile, "[ПОМИЛКА] ", log.Ldate|log.Ltime)
	logger.Printf("Вузол %s недоступний: %v", n.Address, err)
	fmt.Printf("[ПОМИЛКА] Вузол %s недоступний: %v\n", n.Address, err)
}

// Логування доступності вузла.
func (n *Node) logInfo() {
	logFile, _ := os.OpenFile("network_status.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	defer logFile.Close()
	logger := log.New(logFile, "[ІНФО] ", log.Ldate|log.Ltime)
	logger.Printf("Вузол %s доступний.", n.Address)
	fmt.Printf("[ІНФО] Вузол %s доступний.\n", n.Address)
}

// Відправка сповіщення про недоступність вузла з перевіркою часу останнього сповіщення.
func (n *Node) sendAlert(config *Config) {
	n.alertLock.Lock()
	defer n.alertLock.Unlock()

	// Уникнення дублювання сповіщень протягом 10 хвилин
	if time.Since(n.lastAlert) < 10*time.Minute {
		return
	}

	msg := fmt.Sprintf("Від: %s\nДо: %s\nТема: [СПОВІЩЕННЯ] Вузол недоступний\n\nВузол %s недоступний протягом %d спроб. Будь ласка, перевірте підключення.", config.FromEmail, n.AlertEmail, n.Address, n.MaxErrors)

	for i := 0; i < config.RetryCount; i++ {
		err := smtp.SendMail(
			fmt.Sprintf("%s:%s", config.SMTPServer, config.SMTPPort),
			smtp.PlainAuth("", config.Username, config.Password, config.SMTPServer),
			config.FromEmail, []string{n.AlertEmail}, []byte(msg),
		)

		if err != nil {
			log.Printf("[ПОМИЛКА] Не вдалося відправити сповіщення для вузла %s (спроба %d/%d): %v", n.Address, i+1, config.RetryCount, err)
			time.Sleep(time.Duration(config.RetryDelay) * time.Second)
		} else {
			log.Printf("[СПОВІЩЕННЯ] Сповіщення відправлено для вузла %s.", n.Address)
			n.lastAlert = time.Now()
			break
		}
	}
}

// Додавання вузла до чорного списку.
func (n *Node) blacklistNode() {
	n.blacklisted = true
	n.blacklistTime = time.Now()
	log.Printf("[ПОПЕРЕДЖЕННЯ] Вузол %s додано до чорного списку.", n.Address)
}

// Безперервна перевірка вузлів.
func Monitor(nodes []*Node, interval time.Duration, config *Config) {
	for {
		for _, node := range nodes {
			node.Check(config)
		}
		time.Sleep(interval)
	}
}

// Приклад використання.
func main() {
	config, err := LoadConfig("smtp_config.json")
	if err != nil {
		log.Fatalf("Не вдалося завантажити конфіг: %v", err)
	}

	nodes := []*Node{
		{Address: "127.0.0.1:8080", MaxErrors: 3, AlertEmail: "admin@example.com"},
		{Address: "192.168.1.100:8080", MaxErrors: 3, AlertEmail: "admin@example.com"},
		{Address: "example.com:8080", MaxErrors: 3, AlertEmail: "admin@example.com"},
	}

	// Початок моніторингу кожні 10 секунд
	Monitor(nodes, 10*time.Second, config)
}
