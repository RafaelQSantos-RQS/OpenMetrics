use sysinfo::{Components, Disks, Networks, System};
use super::types::*;

pub struct Collector {
    system: System,
    networks: Networks,
    disks: Disks,
    components: Components,
    prev_net_rx: u64,
    prev_net_tx: u64,
}

impl Collector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let networks = Networks::new_with_refreshed_list();
        let disks = Disks::new_with_refreshed_list();

        let prev_net_rx: u64 = networks.iter().map(|(_, d)| d.total_received()).sum();
        let prev_net_tx: u64 = networks.iter().map(|(_, d)| d.total_transmitted()).sum();

        Collector {
            system: sys,
            networks,
            disks,
            components: Components::new_with_refreshed_list(),
            prev_net_rx,
            prev_net_tx,
        }
    }

    pub fn collect(&mut self) -> MetricSnapshot {
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.system.refresh_all();
        self.networks.refresh(true);
        self.disks.refresh(true);

        let cur_net_rx: u64 = self.networks.iter().map(|(_, d)| d.total_received()).sum();
        let cur_net_tx: u64 = self.networks.iter().map(|(_, d)| d.total_transmitted()).sum();

        let rx_rate = cur_net_rx.saturating_sub(self.prev_net_rx);
        let tx_rate = cur_net_tx.saturating_sub(self.prev_net_tx);

        self.prev_net_rx = cur_net_rx;
        self.prev_net_tx = cur_net_tx;

        MetricSnapshot {
            timestamp: chrono::Utc::now(),
            cpu: self.collect_cpu(),
            memory: self.collect_memory(),
            disk: self.collect_disk(),
            network: self.collect_network(rx_rate, tx_rate, cur_net_rx, cur_net_tx),
            processes: self.collect_processes(),
            system: self.collect_system_info(),
        }
    }

    fn collect_cpu(&self) -> CpuMetrics {
        let global_cpu = self.system.global_cpu_usage();
        let per_core: Vec<CoreMetrics> = self.system.cpus().iter().enumerate().map(|(i, c)| {
            CoreMetrics { index: i, usage_percent: c.cpu_usage(), frequency_mhz: c.frequency() }
        }).collect();

        let cpus = self.system.cpus();
        CpuMetrics {
            usage_percent: global_cpu,
            per_core,
            frequency_mhz: cpus.first().map(|c| c.frequency()).unwrap_or(0),
            frequency_min_mhz: cpus.iter().map(|c| c.frequency()).min().unwrap_or(0),
            frequency_max_mhz: cpus.iter().map(|c| c.frequency()).max().unwrap_or(0),
            model_name: cpus.first().map(|c| c.brand().to_string()).unwrap_or_default(),
        }
    }

    fn collect_memory(&self) -> MemoryMetrics {
        MemoryMetrics {
            total_bytes: self.system.total_memory(),
            used_bytes: self.system.used_memory(),
            available_bytes: self.system.available_memory(),
            free_bytes: self.system.free_memory(),
            cached_bytes: 0,
            buffers_bytes: 0,
            shared_bytes: 0,
            swap_total_bytes: self.system.total_swap(),
            swap_used_bytes: self.system.used_swap(),
            swap_free_bytes: self.system.total_swap().saturating_sub(self.system.used_swap()),
        }
    }

    fn collect_disk(&self) -> Vec<DiskMetrics> {
        self.disks.iter().map(|d| {
            DiskMetrics {
                mount_point: d.mount_point().to_string_lossy().to_string(),
                total_bytes: d.total_space(),
                used_bytes: d.total_space().saturating_sub(d.available_space()),
                available_bytes: d.available_space(),
                file_system: d.file_system().to_string_lossy().to_string(),
                read_bytes_per_sec: 0,
                write_bytes_per_sec: 0,
                io_percent: 0.0,
            }
        }).collect()
    }

    fn collect_network(&self, rx_rate: u64, tx_rate: u64, total_rx: u64, total_tx: u64) -> Vec<NetworkMetrics> {
        self.networks.iter().map(|(name, data)| {
            let iface_rx = data.total_received();
            let iface_tx = data.total_transmitted();
            NetworkMetrics {
                interface_name: name.clone(),
                bytes_sent: iface_tx,
                bytes_received: iface_rx,
                packets_sent: data.total_packets_transmitted(),
                packets_received: data.total_packets_received(),
                bytes_sent_per_sec: if total_tx == iface_tx { tx_rate } else { 0 },
                bytes_received_per_sec: if total_rx == iface_rx { rx_rate } else { 0 },
                peak_rx: iface_rx,
                peak_tx: iface_tx,
            }
        }).collect()
    }

    fn collect_processes(&self) -> ProcessMetrics {
        let mut procs: Vec<_> = self.system.processes().values().collect();
        procs.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));

        let total_count = self.system.processes().len();
        let top_by_cpu: Vec<ProcessInfo> = procs.iter().take(30).map(|p| {
            let uptime = System::uptime();
            let start = p.start_time();
            let elapsed = uptime.saturating_sub(start);
            let mins = elapsed / 60;
            let secs = elapsed % 60;

            ProcessInfo {
                pid: p.pid().as_u32(),
                name: p.name().to_string_lossy().to_string(),
                cpu_percent: p.cpu_usage(),
                memory_percent: p.memory() as f32 / self.system.total_memory() as f32 * 100.0,
                memory_bytes: p.memory(),
                threads: 1,
                user: p.user_id().map(|u| u.to_string()).unwrap_or_else(|| "-".into()),
                status: format!("{:?}", p.status()),
                priority: 0,
                start_time: format!("{}m{}s", mins, secs),
            }
        }).collect();

        ProcessMetrics { top_by_cpu, total_count }
    }

    fn collect_system_info(&self) -> SystemInfo {
        let load = System::load_average();
        let temps: Vec<TemperatureInfo> = self.components.iter().filter_map(|c| {
            let label = c.label().to_string();
            let temp = c.temperature();
            if let Some(val) = temp {
                if val > 0.0 {
                    Some(TemperatureInfo { label, celsius: val })
                } else { None }
            } else { None }
        }).collect();

        SystemInfo {
            hostname: System::host_name().unwrap_or_default(),
            os_name: System::name().unwrap_or_default(),
            os_version: System::os_version().unwrap_or_default(),
            kernel_version: System::long_os_version().unwrap_or_default(),
            uptime_secs: System::uptime(),
            load_avg_1: load.one,
            load_avg_5: load.five,
            load_avg_15: load.fifteen,
            temperatures: temps,
        }
    }
}
