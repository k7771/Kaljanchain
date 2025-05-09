package netmon

import (
	"fmt"
	"net"
	"time"
	"log"
	"os"
	"net/smtp"
)

// Node represents a blockchain node with its address and status.
type Node struct {
	Address     string
	Status      bool
	ErrorCount  int
	MaxErrors   int
	AlertEmail  string
}

// Check verifies if the node is reachable and updates its status.
func (n *Node) Check() {
	conn, err := net.DialTimeout("tcp", n.Address, 5*time.Second)
	if err != nil {
		n.Status = false
		n.ErrorCount++
		n.logError(err)
		if n.ErrorCount >= n.MaxErrors {
			n.sendAlert()
			n.ErrorCount = 0 // Reset error count after alert
		}
	} else {
		n.Status = true
		n.ErrorCount = 0
		conn.Close()
		n.logInfo()
	}
}

// logError logs the unreachable status of a node.
func (n *Node) logError(err error) {
	logFile, _ := os.OpenFile("network_errors.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	defer logFile.Close()
	logger := log.New(logFile, "[ERROR] ", log.Ldate|log.Ltime)
	logger.Printf("Node %s is unreachable: %v", n.Address, err)
	fmt.Printf("[ERROR] Node %s is unreachable: %v\n", n.Address, err)
}

// logInfo logs the reachable status of a node.
func (n *Node) logInfo() {
	logFile, _ := os.OpenFile("network_status.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	defer logFile.Close()
	logger := log.New(logFile, "[INFO] ", log.Ldate|log.Ltime)
	logger.Printf("Node %s is reachable.", n.Address)
	fmt.Printf("[INFO] Node %s is reachable.\n", n.Address)
}

// sendAlert sends an email alert when a node is repeatedly unreachable.
func (n *Node) sendAlert() {
	from := "kaljanchain.alerts@example.com"
	password := "your_password"
	recipient := n.AlertEmail
	subject := "[ALERT] Node Unreachable"
	body := fmt.Sprintf("Node %s has been unreachable for %d consecutive checks. Please investigate.", n.Address, n.MaxErrors)

	msg := fmt.Sprintf("From: %s\nTo: %s\nSubject: %s\n\n%s", from, recipient, subject, body)

	err := smtp.SendMail("smtp.example.com:587",
		smtp.PlainAuth("", from, password, "smtp.example.com"),
		from, []string{recipient}, []byte(msg))

	if err != nil {
		log.Printf("[ERROR] Failed to send alert for node %s: %v", n.Address, err)
	} else {
		log.Printf("[ALERT] Email alert sent for node %s.", n.Address)
	}
}

// Monitor continuously checks the status of a list of nodes.
func Monitor(nodes []*Node, interval time.Duration) {
	for {
		for _, node := range nodes {
			node.Check()
		}
		time.Sleep(interval)
	}
}

// Example usage
func main() {
	nodes := []*Node{
		{Address: "127.0.0.1:8080", MaxErrors: 3, AlertEmail: "admin@example.com"},
		{Address: "192.168.1.100:8080", MaxErrors: 3, AlertEmail: "admin@example.com"},
		{Address: "example.com:8080", MaxErrors: 3, AlertEmail: "admin@example.com"},
	}

	// Start monitoring nodes every 10 seconds
	Monitor(nodes, 10*time.Second)
}
