pub mod collector;
pub mod types;

pub use types::*;

use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::time::{interval, Duration};

const HISTORY_SIZE: usize = 300; // 5 minutes at 1s intervals

#[derive(Clone, Debug)]
pub struct MetricsHistory {
    pub cpu: VecDeque<f32>,
    pub memory: VecDeque<f32>,
    pub network_rx: VecDeque<f32>,
    pub network_tx: VecDeque<f32>,
    pub disk_read: VecDeque<f32>,
    pub disk_write: VecDeque<f32>,
}

impl MetricsHistory {
    fn new() -> Self {
        MetricsHistory {
            cpu: VecDeque::with_capacity(HISTORY_SIZE),
            memory: VecDeque::with_capacity(HISTORY_SIZE),
            network_rx: VecDeque::with_capacity(HISTORY_SIZE),
            network_tx: VecDeque::with_capacity(HISTORY_SIZE),
            disk_read: VecDeque::with_capacity(HISTORY_SIZE),
            disk_write: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    fn push(&mut self, snapshot: &MetricSnapshot) {
        let push_trim = |dq: &mut VecDeque<_>, val: f32| {
            if dq.len() >= HISTORY_SIZE { dq.pop_front(); }
            dq.push_back(val);
        };

        push_trim(&mut self.cpu, snapshot.cpu.usage_percent);

        let mem_pct = if snapshot.memory.total_bytes > 0 {
            snapshot.memory.used_bytes as f32 / snapshot.memory.total_bytes as f32 * 100.0
        } else { 0.0 };
        push_trim(&mut self.memory, mem_pct);

        let total_rx: u64 = snapshot.network.iter().map(|n| n.bytes_received_per_sec).sum();
        let total_tx: u64 = snapshot.network.iter().map(|n| n.bytes_sent_per_sec).sum();
        push_trim(&mut self.network_rx, total_rx as f32);
        push_trim(&mut self.network_tx, total_tx as f32);

        let total_read: u64 = snapshot.disk.iter().map(|d| d.read_bytes_per_sec).sum();
        let total_write: u64 = snapshot.disk.iter().map(|d| d.write_bytes_per_sec).sum();
        push_trim(&mut self.disk_read, total_read as f32);
        push_trim(&mut self.disk_write, total_write as f32);
    }
}

pub type MetricsState = Arc<RwLock<MetricsData>>;

#[derive(Clone)]
pub struct MetricsData {
    pub latest: Option<MetricSnapshot>,
    pub history: MetricsHistory,
}

impl MetricsData {
    fn new() -> Self {
        MetricsData { latest: None, history: MetricsHistory::new() }
    }
}

pub fn new_metrics_state() -> MetricsState {
    Arc::new(RwLock::new(MetricsData::new()))
}

pub fn spawn_collector(state: MetricsState, interval_secs: u64) {
    tokio::spawn(async move {
        let mut collector = collector::Collector::new();
        let mut timer = interval(Duration::from_secs(interval_secs));

        let snapshot = collector.collect();
        if let Ok(mut data) = state.write() {
            data.history.push(&snapshot);
            data.latest = Some(snapshot);
        }

        loop {
            timer.tick().await;
            let snapshot = collector.collect();
            if let Ok(mut data) = state.write() {
                data.history.push(&snapshot);
                data.latest = Some(snapshot);
            }
        }
    });
}
