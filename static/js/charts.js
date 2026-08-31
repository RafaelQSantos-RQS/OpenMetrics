/* Chart manager - initializes sparklines from data-chart attributes */
const _charts = {};

function _parse(str) {
    try { const p = JSON.parse(str); return Array.isArray(p) ? p : []; } catch(e) { return []; }
}

function _getOrCreate(id, cfg) {
    const ctx = document.getElementById(id);
    if (!ctx) return;
    if (_charts[id]) _charts[id].destroy();
    _charts[id] = new Chart(ctx, cfg);
}

function _initCpuChart(el) {
    const data = _parse(el.dataset.history);
    if (!data.length) return;
    _getOrCreate('cpu-spark', {
        type: 'line',
        data: { labels: data.map(() => ''), datasets: [{ data: data, borderColor: '#3fb950', backgroundColor: 'rgba(63,185,80,0.12)', fill: true, tension: 0.3, pointRadius: 0, borderWidth: 1.5 }] },
        options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false }, tooltip: { enabled: false } }, scales: { x: { display: false }, y: { display: false, min: 0, max: 100 } }, animation: false }
    });
}

function _initMemChart(el) {
    const data = _parse(el.dataset.history);
    if (!data.length) return;
    _getOrCreate('mem-spark', {
        type: 'line',
        data: { labels: data.map(() => ''), datasets: [{ data: data, borderColor: '#bc8cff', backgroundColor: 'rgba(188,140,255,0.12)', fill: true, tension: 0.3, pointRadius: 0, borderWidth: 1.5 }] },
        options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false }, tooltip: { enabled: false } }, scales: { x: { display: false }, y: { display: false, min: 0, max: 100 } }, animation: false }
    });
}

function _initNetChart(el) {
    const rx = _parse(el.dataset.rxHistory);
    const tx = _parse(el.dataset.txHistory);
    if (!rx.length) return;
    _getOrCreate('net-spark', {
        type: 'line',
        data: { labels: rx.map(() => ''), datasets: [
            { data: rx, borderColor: '#58a6ff', backgroundColor: 'rgba(88,166,255,0.08)', fill: true, tension: 0.3, pointRadius: 0, borderWidth: 1 },
            { data: tx, borderColor: '#bc8cff', backgroundColor: 'rgba(188,140,255,0.08)', fill: true, tension: 0.3, pointRadius: 0, borderWidth: 1 }
        ] },
        options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false }, tooltip: { enabled: false } }, scales: { x: { display: false }, y: { display: false } }, animation: false }
    });
}

function _initDiskChart(el) {
    const r = _parse(el.dataset.readHistory);
    const w = _parse(el.dataset.writeHistory);
    if (!r.length) return;
    _getOrCreate('disk-spark', {
        type: 'line',
        data: { labels: r.map(() => ''), datasets: [
            { data: r, borderColor: '#e3b341', backgroundColor: 'rgba(227,179,65,0.08)', fill: true, tension: 0.3, pointRadius: 0, borderWidth: 1 },
            { data: w, borderColor: '#f85149', backgroundColor: 'rgba(248,81,73,0.08)', fill: true, tension: 0.3, pointRadius: 0, borderWidth: 1 }
        ] },
        options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false }, tooltip: { enabled: false } }, scales: { x: { display: false }, y: { display: false } }, animation: false }
    });
}

/* Listen for HTMX swaps */
document.body.addEventListener('htmx:afterSwap', function(e) {
    const target = e.detail.target || e.detail.elt;
    if (!target) return;

    const cpuEl = target.querySelector ? target.querySelector('[data-chart="cpu"]') : null;
    if (cpuEl) _initCpuChart(cpuEl);

    const memEl = target.querySelector ? target.querySelector('[data-chart="memory"]') : null;
    if (memEl) _initMemChart(memEl);

    const netEl = target.querySelector ? target.querySelector('[data-chart="network"]') : null;
    if (netEl) _initNetChart(netEl);

    const diskEl = target.querySelector ? target.querySelector('[data-chart="disk"]') : null;
    if (diskEl) _initDiskChart(diskEl);
});

/* Also init on page load for non-HTMX content */
document.addEventListener('DOMContentLoaded', function() {
    document.querySelectorAll('[data-chart]').forEach(function(el) {
        const type = el.dataset.chart;
        if (type === 'cpu') _initCpuChart(el);
        else if (type === 'memory') _initMemChart(el);
        else if (type === 'network') _initNetChart(el);
        else if (type === 'disk') _initDiskChart(el);
    });
});
