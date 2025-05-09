// Історія змін статусів вузлів
const history = {};
const MAX_HISTORY_ENTRIES = 100;
const charts = {};

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

    // Оновлення графіка, якщо він відкритий
    if (charts[node.address]) {
        updateChart(node.address);
    }
}

// Повертає історію вузла
function getNodeHistory(address) {
    return history[address] || [];
}

// Оновлює графік для вузла
function updateChart(address) {
    const chart = charts[address];
    const nodeHistory = getNodeHistory(address);
    const labels = nodeHistory.map(entry => entry.timestamp);
    const data = nodeHistory.map(entry => entry.errorCount);
    chart.data.labels = labels;
    chart.data.datasets[0].data = data;
    chart.update();
}

// Створює новий графік для вузла
function createChart(address, labels, data) {
    const ctx = document.getElementById('historyChart').getContext('2d');
    charts[address] = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [{
                label: 'Кількість помилок',
                data: data,
                borderColor: 'red',
                fill: false
            }]
        },
        options: {
            responsive: true,
            scales: {
                x: { display: true },
                y: { display: true }
            }
        }
    });
}
