use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSnapshot {
    pub timestamp: DateTime<Utc>,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub disk: Vec<DiskMetrics>,
    pub network: Vec<NetworkMetrics>,
    pub processes: ProcessMetrics,
    pub system: SystemInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub usage_percent: f32,
    pub per_core: Vec<CoreMetrics>,
    pub frequency_mhz: u64,
    pub frequency_min_mhz: u64,
    pub frequency_max_mhz: u64,
    pub model_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMetrics {
    pub index: usize,
    pub usage_percent: f32,
    pub frequency_mhz: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub free_bytes: u64,
    pub cached_bytes: u64,
    pub buffers_bytes: u64,
    pub shared_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_free_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    pub mount_point: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub file_system: String,
    pub read_bytes_per_sec: u64,
    pub write_bytes_per_sec: u64,
    pub io_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub interface_name: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub bytes_sent_per_sec: u64,
    pub bytes_received_per_sec: u64,
    pub peak_rx: u64,
    pub peak_tx: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMetrics {
    pub top_by_cpu: Vec<ProcessInfo>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub memory_bytes: u64,
    pub threads: u32,
    pub user: String,
    pub status: String,
    pub priority: i32,
    pub start_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub uptime_secs: u64,
    pub load_avg_1: f64,
    pub load_avg_5: f64,
    pub load_avg_15: f64,
    pub temperatures: Vec<TemperatureInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureInfo {
    pub label: String,
    pub celsius: f32,
}
