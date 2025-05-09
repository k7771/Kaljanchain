// Історія змін статусів вузлів
const history = {};
const MAX_HISTORY_ENTRIES = 100;

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

    // Очищення старих записів, якщо їх більше ніж MAX_HISTORY_ENTRIES
    if (history[node.address].length > MAX_HISTORY_ENTRIES) {
        history[node.address].shift();
    }
}

// Повертає історію вузла
function getNodeHistory(address) {
    return history[address] || [];
}

// Повертає всі вузли з історією
function getAllHistory() {
    return history;
}
