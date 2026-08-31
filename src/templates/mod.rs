use askama::Template;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

#[derive(Template)]
#[template(path = "partials/header.html")]
pub struct HeaderTemplate {
    pub hostname: String,
    pub uptime: String,
    pub load: String,
    pub cores: String,
}

#[derive(Template)]
#[template(path = "partials/cpu_panel.html")]
pub struct CpuPanelTemplate {
    pub usage: String,
    pub cores: Vec<CoreInfo>,
    pub load_1: String,
    pub load_5: String,
    pub load_15: String,
    pub temps: Vec<TempInfo>,
    pub freq: String,
    pub freq_max: String,
    pub history: String,
}

pub struct CoreInfo {
    pub index: usize,
    pub usage: f32,
    pub usage_display: String,
    pub bar_class: String,
}

pub struct TempInfo {
    pub label: String,
    pub celsius: String,
    pub temp_class: String,
}

#[derive(Template)]
#[template(path = "partials/memory_panel.html")]
pub struct MemoryPanelTemplate {
    pub usage: String,
    pub used: String,
    pub available: String,
    pub free: String,
    pub cached: String,
    pub buffers: String,
    pub shared: String,
    pub total: String,
    pub swap_used: String,
    pub swap_free: String,
    pub swap_total: String,
    pub swap_pct: String,
    pub history: String,
}

#[derive(Template)]
#[template(path = "partials/disk_panel.html")]
pub struct DiskPanelTemplate {
    pub disks: Vec<DiskInfo>,
    pub read_history: String,
    pub write_history: String,
}

pub struct DiskInfo {
    pub mount: String,
    pub used: String,
    pub total: String,
    pub read: String,
    pub write: String,
    pub io: String,
    pub pct: f64,
    pub pct_display: String,
}

#[derive(Template)]
#[template(path = "partials/network_panel.html")]
pub struct NetworkPanelTemplate {
    pub rx_rate: String,
    pub tx_rate: String,
    pub rx_total: String,
    pub tx_total: String,
    pub rx_peak: String,
    pub tx_peak: String,
    pub packets_rx: String,
    pub packets_tx: String,
    pub rx_history: String,
    pub tx_history: String,
}

#[derive(Template)]
#[template(path = "partials/process_table.html")]
pub struct ProcessTableTemplate {
    pub processes: Vec<ProcessInfo>,
    pub count: String,
}

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub cpu_display: String,
    pub mem_display: String,
    pub threads: String,
    pub user: String,
    pub priority: String,
    pub start_time: String,
    pub status: String,
    pub status_short: String,
    pub status_class: String,
}
