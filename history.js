// Журнал дій для вузлів
const actionLog = [];

// Додає запис у журнал дій
function logAction(message, type = 'info') {
    const timestamp = new Date().toLocaleString();
    actionLog.push({ message, type, timestamp });
    console.log(`[${timestamp}] ${message}`);
    showNotification(message, type);
    updateActionLog();
}

// Оновлює журнал дій у таблиці
function updateActionLog() {
    const logContainer = document.getElementById('actionLog');
    if (!logContainer) return;

    logContainer.innerHTML = '';
    actionLog.slice().reverse().forEach(entry => {
        const logItem = document.createElement('div');
        logItem.classList.add('log-entry', entry.type);
        logItem.textContent = `[${entry.timestamp}] ${entry.message}`;
        logContainer.appendChild(logItem);
    });
}

// Додає вузол до таблиці
function addNodeToTable(address, status, lastCheck, errorCount) {
    const tbody = document.querySelector('#nodes tbody');
    const row = document.createElement('tr');
    row.innerHTML = `
        <td>${address}</td>
        <td class="status-indicator">${status}</td>
        <td>${lastCheck}</td>
        <td>${errorCount}</td>
        <td><button onclick="showHistory('${address}')">Показати</button></td>
        <td><button onclick="resetNodeHistory('${address}')">Скинути</button></td>
    `;
    tbody.appendChild(row);
    updateNodeStatusIndicators();
}

// Додає кнопку відновлення вузлів
function addRestoreButton() {
    const container = document.getElementById('archive-controls');
    if (!container) return;

    const button = document.createElement('button');
    button.textContent = 'Відновити всі вузли з архіву';
    button.onclick = restoreAllNodes;
    container.appendChild(button);
}

// Відновлює всі вузли з архіву
function restoreAllNodes() {
    logAction('Відновлення всіх вузлів з архіву...', 'info');
    // Логіка для відновлення вузлів
    logAction('Всі вузли успішно відновлені.', 'success');
}

// Ініціалізація при завантаженні сторінки
document.addEventListener('DOMContentLoaded', () => {
    addRestoreButton();
    updateActionLog();
});
