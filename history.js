// Історія змін статусів вузлів
const history = {};
const MAX_HISTORY_ENTRIES = 100;
const charts = {};
let NODE_INACTIVITY_LIMIT = 2592000000; // 30 днів за замовчуванням

// Додає запис до історії вузла
function addToHistory(node) {
    if (!history[node.address]) {
        history[node.address] = [];
    }
    const timestamp = new Date().toLocaleString();
    const entry = {
        status: node.status,
        timestamp: timestamp,
        errorCount: node.error_count,
        time: Date.now()
    };
    history[node.address].push(entry);

    // Очищення старих записів, якщо їх більше ніж MAX_HISTORY_ENTRIES
    if (history[node.address].length > getMaxHistoryEntries()) {
        history[node.address].shift();
    }

    // Видалення вузлів з нульовою активністю
    if (history[node.address].length === 0) {
        delete history[node.address];
    }

    // Оновлення графіка, якщо він відкритий
    if (charts[node.address]) {
        updateChart(node.address);
    }

    // Автоматичне видалення неактивних вузлів
    cleanupInactiveNodes();
}

// Очищує вузли з нульовою активністю
function cleanupInactiveNodes() {
    const cutoff = Date.now() - NODE_INACTIVITY_LIMIT;
    for (const address in history) {
        const lastActivity = history[address].at(-1)?.time || 0;
        if (lastActivity < cutoff) {
            archiveNodeHistory(address);
            delete history[address];
            console.log(`Вузол ${address} видалений через неактивність.`);
            showNotification(`Вузол ${address} видалений через неактивність.`);
        }
    }
}

// Архівує історію вузла перед видаленням
function archiveNodeHistory(address) {
    let csv = 'Адреса,Статус,Час,Кількість помилок\n';
    history[address].forEach(entry => {
        csv += `${address},${entry.status},${entry.timestamp},${entry.errorCount}\n`;
    });
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${address}_history_backup.csv`;
    a.click();
    URL.revokeObjectURL(url);
    console.log(`Історія вузла ${address} архівована.`);
    showNotification(`Історія вузла ${address} архівована.`);
}

// Масове архівування всіх вузлів
function archiveAllNodes() {
    for (const address in history) {
        archiveNodeHistory(address);
    }
    showNotification("Історії всіх вузлів успішно архівовані.");
    createBackupArchive();
}

// Створює резервну копію всієї історії вузлів
function createBackupArchive() {
    let csv = 'Адреса,Статус,Час,Кількість помилок\n';
    for (const address in history) {
        history[address].forEach(entry => {
            csv += `${address},${entry.status},${entry.timestamp},${entry.errorCount}\n`;
        });
    }
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `nodes_backup_${new Date().toISOString().slice(0, 10)}.csv`;
    a.click();
    URL.revokeObjectURL(url);
    showNotification("Резервна копія всіх вузлів успішно створена.");
}

// Масове очищення історії всіх вузлів
function resetAllNodes() {
    for (const address in history) {
        resetNodeHistory(address);
    }
    showNotification("Історії всіх вузлів успішно очищені.");
}

// Відображає індикатори стану вузлів
function updateNodeStatusIndicators() {
    const rows = document.querySelectorAll('#nodes tbody tr');
    rows.forEach(row => {
        const statusCell = row.cells[1];
        const status = statusCell.textContent.trim();
        if (status === 'доступний') {
            statusCell.classList.add('available');
            statusCell.classList.remove('unavailable', 'inactive');
        } else if (status === 'недоступний') {
            statusCell.classList.add('unavailable');
            statusCell.classList.remove('available', 'inactive');
        } else {
            statusCell.classList.add('inactive');
            statusCell.classList.remove('available', 'unavailable');
        }
    });
}

// Повертає історію вузла з фільтрацією за часом
function getNodeHistory(address, range = 86400000) { // за замовчуванням 24 години
    const cutoff = Date.now() - range;
    return (history[address] || []).filter(entry => entry.time >= cutoff);
}

// Оновлює графік для вузла
function updateChart(address) {
    const chart = charts[address];
    const nodeHistory = getNodeHistory(address, getSelectedRange());
    const labels = nodeHistory.map(entry => new Date(entry.time).toLocaleString());
    const data = nodeHistory.map(entry => entry.errorCount);
    chart.data.labels = labels;
    chart.data.datasets[0].data = data;
    chart.options.scales.y.suggestedMin = Math.min(...data) - 1;
    chart.options.scales.y.suggestedMax = Math.max(...data) + 1;
    chart.update();

    // Оновлення статистики
    updateStats(address, nodeHistory);
}

// Повертає максимальну кількість записів в історії
function getMaxHistoryEntries() {
    const maxEntries = document.getElementById('maxHistoryEntries').value;
    return parseInt(maxEntries) || MAX_HISTORY_ENTRIES;
}

// Відображає повідомлення
function showNotification(message) {
    alert(message);
}
