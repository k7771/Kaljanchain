// Історія змін статусів вузлів
const history = {};

// Додає запис до історії вузла
function addToHistory(node) {
    if (!history[node.address]) {
        history[node.address] = [];
    }
    const timestamp = new Date().toLocaleString();
    const entry = {
        status: node.status,
        timestamp: timestamp,
        errorCount: node.error_count
    };
    history[node.address].push(entry);
}

// Повертає історію вузла
function getNodeHistory(address) {
    return history[address] || [];
}

// Повертає всі вузли з історією
function getAllHistory() {
    return history;
}
