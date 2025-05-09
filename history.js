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
        removeNodeFromTable(node.address);
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
            removeNodeFromTable(address);
            showNotification(`Вузол ${address} видалений через неактивність.`, 'error');
        }
    }
}

// Видаляє вузол з таблиці
function removeNodeFromTable(address) {
    const rows = document.querySelectorAll('#nodes tbody tr');
    rows.forEach(row => {
        if (row.cells[0].textContent.trim() === address) {
            row.remove();
        }
    });
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
    showNotification(`Історія вузла ${address} архівована.`, 'success');
}

// Масове архівування всіх вузлів
function archiveAllNodes() {
    for (const address in history) {
        archiveNodeHistory(address);
        delete history[address];
        removeNodeFromTable(address);
    }
    createBackupArchive();
    showNotification("Історії всіх вузлів успішно архівовані та очищені.", 'success');
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
    showNotification("Резервна копія всіх вузлів успішно створена.", 'success');
}

// Масове очищення історії всіх вузлів
function resetAllNodes() {
    for (const address in history) {
        resetNodeHistory(address);
        removeNodeFromTable(address);
    }
    showNotification("Історії всіх вузлів успішно очищені.", 'success');
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

// Відображає повідомлення з різними стилями
function showNotification(message, type = 'info') {
    const notification = document.createElement('div');
    notification.classList.add('notification', type);
    notification.textContent = message;
    document.body.appendChild(notification);
    setTimeout(() => {
        notification.remove();
    }, 5000);
}
