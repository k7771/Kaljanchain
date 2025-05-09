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
        errorCount: node.error_count,
        time: Date.now()
    };
    history[node.address].push(entry);

    // Очищення старих записів, якщо їх більше ніж MAX_HISTORY_ENTRIES
    if (history[node.address].length > MAX_HISTORY_ENTRIES) {
        history[node.address].shift();
    }

    // Оновлення графіка, якщо він відкритий
    if (charts[node.address]) {
        updateChart(address);
    }
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

// Створює новий графік для вузла
function createChart(address) {
    const ctx = document.getElementById('historyChart').getContext('2d');
    charts[address] = new Chart(ctx, {
        type: 'line',
        data: {
            labels: [],
            datasets: [{
                label: 'Кількість помилок',
                data: [],
                borderColor: 'red',
                fill: false
            }]
        },
        options: {
            responsive: true,
            scales: {
                x: { display: true },
                y: { display: true, beginAtZero: false }
            }
        }
    });
    updateChart(address);
}

// Повертає вибраний діапазон часу
function getSelectedRange() {
    const range = document.getElementById('timeRange').value;
    switch (range) {
        case '1h': return 3600000;
        case '6h': return 21600000;
        case '12h': return 43200000;
        case '24h': return 86400000;
        case '7d': return 604800000;
        case '30d': return 2592000000;
        default: return 86400000;
    }
}

// Оновлює підсумкову статистику для вузла
function updateStats(address, nodeHistory) {
    const totalErrors = nodeHistory.reduce((sum, entry) => sum + entry.errorCount, 0);
    const minErrors = Math.min(...nodeHistory.map(entry => entry.errorCount));
    const maxErrors = Math.max(...nodeHistory.map(entry => entry.errorCount));
    const avgErrors = (totalErrors / nodeHistory.length).toFixed(2);

    const statsElement = document.getElementById('nodeStats');
    statsElement.innerText = `Вузол: ${address} | Загальна кількість помилок: ${totalErrors} | Мінімум: ${minErrors} | Максимум: ${maxErrors} | Середнє: ${avgErrors}`;
}

// Завантажує графік у PNG форматі
function downloadChartAsPNG(address) {
    const link = document.createElement('a');
    link.href = charts[address].toBase64Image();
    link.download = `${address}_history.png`;
    link.click();
}

// Завантажує графік у PDF форматі
function downloadChartAsPDF(address) {
    const canvas = charts[address].canvas;
    const imgData = canvas.toDataURL('image/png');
    const pdfWindow = window.open();
    pdfWindow.document.write('<iframe src="' + imgData + '" width="100%" height="100%"></iframe>');
}
