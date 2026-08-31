use axum::{extract::State, response::Html};
use askama::Template;
use crate::templates::*;
use crate::AppState;

fn fmt_bytes(b: u64) -> String {
    if b == 0 { return "0 B".into(); }
    let k = 1024_f64;
    let s = ["B","KB","MB","GB","TB"];
    let i = (b as f64).log(k.floor()).floor() as usize;
    format!("{:.1} {}", b as f64 / k.powi(i as i32), s[i.min(4)])
}

fn fmt_rate(b: u64) -> String {
    if b == 0 { return "0 B/s".into(); }
    let k = 1024_f64;
    let s = ["B/s","KB/s","MB/s","GB/s"];
    let i = (b as f64).log(k.floor()).floor() as usize;
    format!("{:.1} {}", b as f64 / k.powi(i as i32), s[i.min(3)])
}

fn fmt_uptime(s: u64) -> String {
    let d = s / 86400; let h = (s % 86400) / 3600; let m = (s % 3600) / 60;
    if d > 0 { format!("{}d {}h", d, h) }
    else if h > 0 { format!("{}h {}m", h, m) }
    else { format!("{}m", m) }
}

pub async fn dashboard_page() -> Html<String> {
    Html(DashboardTemplate.render().unwrap_or_else(|e| format!("Error: {}", e)))
}

pub async fn header_panel(State(state): State<AppState>) -> Html<String> {
    let html = if let Ok(d) = state.metrics.read() {
        if let Some(ref s) = d.latest {
            HeaderTemplate {
                hostname: s.system.hostname.clone(),
                uptime: fmt_uptime(s.system.uptime_secs),
                load: format!("{:.2}", s.system.load_avg_1),
                cores: s.cpu.per_core.len().to_string(),
            }.render().unwrap_or_else(|e| format!("Error: {}", e))
        } else { "<span>Loading...</span>".into() }
    } else { "<span>Error</span>".into() };
    Html(html)
}

pub async fn cpu_panel(State(state): State<AppState>) -> Html<String> {
    let html = if let Ok(d) = state.metrics.read() {
        let history: Vec<String> = d.history.cpu.iter().map(|v| format!("{:.1}", v)).collect();
        let current = d.history.cpu.back().copied().unwrap_or(0.0);
        if let Some(ref s) = d.latest {
            let cores: Vec<CoreInfo> = s.cpu.per_core.iter().map(|c| {
                let bc = if c.usage_percent > 80.0 { "critical" } else if c.usage_percent > 50.0 { "hot" } else { "" };
                CoreInfo { index: c.index, usage: c.usage_percent, usage_display: format!("{:.0}", c.usage_percent), bar_class: bc.to_string() }
            }).collect();
            let temps: Vec<TempInfo> = s.system.temperatures.iter().map(|t| {
                let tc = if t.celsius > 80.0 { "critical" } else if t.celsius > 60.0 { "hot" } else { "" };
                TempInfo { label: t.label.clone(), celsius: format!("{:.0}", t.celsius), temp_class: tc.to_string() }
            }).collect();
            CpuPanelTemplate {
                usage: format!("{:.1}", current),
                cores,
                load_1: format!("{:.2}", s.system.load_avg_1),
                load_5: format!("{:.2}", s.system.load_avg_5),
                load_15: format!("{:.2}", s.system.load_avg_15),
                temps,
                freq: s.cpu.frequency_mhz.to_string(),
                freq_max: s.cpu.frequency_max_mhz.to_string(),
                history: format!("[{}]", history.join(",")),
            }.render().unwrap_or_else(|e| format!("Error: {}", e))
        } else { "<div class='loading'>Waiting...</div>".into() }
    } else { "<div class='loading'>Error</div>".into() };
    Html(html)
}

pub async fn memory_panel(State(state): State<AppState>) -> Html<String> {
    let html = if let Ok(d) = state.metrics.read() {
        let history: Vec<String> = d.history.memory.iter().map(|v| format!("{:.1}", v)).collect();
        let current = d.history.memory.back().copied().unwrap_or(0.0);
        if let Some(ref s) = d.latest {
            let m = &s.memory;
            let swap_pct = if m.swap_total_bytes > 0 { m.swap_used_bytes as f64 / m.swap_total_bytes as f64 * 100.0 } else { 0.0 };
            MemoryPanelTemplate {
                usage: format!("{:.1}", current),
                used: fmt_bytes(m.used_bytes),
                available: fmt_bytes(m.available_bytes),
                free: fmt_bytes(m.free_bytes),
                cached: fmt_bytes(m.cached_bytes),
                buffers: fmt_bytes(m.buffers_bytes),
                shared: fmt_bytes(m.shared_bytes),
                total: fmt_bytes(m.total_bytes),
                swap_used: fmt_bytes(m.swap_used_bytes),
                swap_free: fmt_bytes(m.swap_free_bytes),
                swap_total: fmt_bytes(m.swap_total_bytes),
                swap_pct: format!("{:.0}", swap_pct),
                history: format!("[{}]", history.join(",")),
            }.render().unwrap_or_else(|e| format!("Error: {}", e))
        } else { "<div class='loading'>Waiting...</div>".into() }
    } else { "<div class='loading'>Error</div>".into() };
    Html(html)
}

pub async fn disk_panel(State(state): State<AppState>) -> Html<String> {
    let html = if let Ok(d) = state.metrics.read() {
        let r_hist: Vec<String> = d.history.disk_read.iter().map(|v| format!("{:.0}", v)).collect();
        let w_hist: Vec<String> = d.history.disk_write.iter().map(|v| format!("{:.0}", v)).collect();
        if let Some(ref s) = d.latest {
            let disks: Vec<DiskInfo> = s.disk.iter().map(|dk| {
                let pct = if dk.total_bytes > 0 { dk.used_bytes as f64 / dk.total_bytes as f64 * 100.0 } else { 0.0 };
                DiskInfo {
                    mount: dk.mount_point.clone(),
                    used: fmt_bytes(dk.used_bytes),
                    total: fmt_bytes(dk.total_bytes),
                    read: fmt_rate(dk.read_bytes_per_sec),
                    write: fmt_rate(dk.write_bytes_per_sec),
                    io: format!("{:.0}", dk.io_percent),
                    pct,
                    pct_display: format!("{:.0}", pct),
                }
            }).collect();
            DiskPanelTemplate { disks, read_history: format!("[{}]", r_hist.join(",")), write_history: format!("[{}]", w_hist.join(",")) }
                .render().unwrap_or_else(|e| format!("Error: {}", e))
        } else { "<div class='loading'>Waiting...</div>".into() }
    } else { "<div class='loading'>Error</div>".into() };
    Html(html)
}

pub async fn network_panel(State(state): State<AppState>) -> Html<String> {
    let html = if let Ok(d) = state.metrics.read() {
        let rx_hist: Vec<String> = d.history.network_rx.iter().map(|v| format!("{:.0}", v)).collect();
        let tx_hist: Vec<String> = d.history.network_tx.iter().map(|v| format!("{:.0}", v)).collect();
        if let Some(ref s) = d.latest {
            let total_rx: u64 = s.network.iter().map(|n| n.bytes_received_per_sec).sum();
            let total_tx: u64 = s.network.iter().map(|n| n.bytes_sent_per_sec).sum();
            let tot_rx: u64 = s.network.iter().map(|n| n.bytes_received).sum();
            let tot_tx: u64 = s.network.iter().map(|n| n.bytes_sent).sum();
            let pk_rx: u64 = s.network.iter().map(|n| n.packets_received).sum();
            let pk_tx: u64 = s.network.iter().map(|n| n.packets_sent).sum();
            NetworkPanelTemplate {
                rx_rate: fmt_rate(total_rx),
                tx_rate: fmt_rate(total_tx),
                rx_total: fmt_bytes(tot_rx),
                tx_total: fmt_bytes(tot_tx),
                rx_peak: fmt_rate(total_rx),
                tx_peak: fmt_rate(total_tx),
                packets_rx: pk_rx.to_string(),
                packets_tx: pk_tx.to_string(),
                rx_history: format!("[{}]", rx_hist.join(",")),
                tx_history: format!("[{}]", tx_hist.join(",")),
            }.render().unwrap_or_else(|e| format!("Error: {}", e))
        } else { "<div class='loading'>Waiting...</div>".into() }
    } else { "<div class='loading'>Error</div>".into() };
    Html(html)
}

pub async fn process_table(State(state): State<AppState>) -> Html<String> {
    let html = if let Ok(d) = state.metrics.read() {
        if let Some(ref s) = d.latest {
            let procs: Vec<ProcessInfo> = s.processes.top_by_cpu.iter().map(|p| {
                let st = p.status.replace('"', "");
                let (sc, ss) = if st == "Run" { ("run", "R") } else { ("sleep", "S") };
                ProcessInfo {
                    pid: p.pid,
                    name: p.name.clone(),
                    cpu: p.cpu_percent,
                    cpu_display: format!("{:.1}", p.cpu_percent),
                    mem_display: format!("{:.1}", p.memory_percent),
                    threads: p.threads.to_string(),
                    user: p.user.clone(),
                    priority: p.priority.to_string(),
                    start_time: p.start_time.clone(),
                    status: st,
                    status_short: ss.to_string(),
                    status_class: sc.to_string(),
                }
            }).collect();
            ProcessTableTemplate { count: s.processes.total_count.to_string(), processes: procs }
                .render().unwrap_or_else(|e| format!("Error: {}", e))
        } else { "<div class='loading'>Waiting...</div>".into() }
    } else { "<div class='loading'>Error</div>".into() };
    Html(html)
}
