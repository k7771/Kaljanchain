package netmon

import (
	"fmt"
	"net"
	"time"
	"log"
)

// Node represents a blockchain node with its address and status.
type Node struct {
	Address string
	Status  bool
}

// Check verifies if the node is reachable and updates its status.
func (n *Node) Check() {
	conn, err := net.DialTimeout("tcp", n.Address, 5*time.Second)
	if err != nil {
		n.Status = false
		log.Printf("[ERROR] Node %s is unreachable: %v", n.Address, err)
	} else {
		n.Status = true
		conn.Close()
		log.Printf("[INFO] Node %s is reachable.", n.Address)
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
		{Address: "127.0.0.1:8080"},
		{Address: "192.168.1.100:8080"},
		{Address: "example.com:8080"},
	}

	// Start monitoring nodes every 10 seconds
	Monitor(nodes, 10*time.Second)
}
