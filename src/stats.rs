use sysinfo::System;

pub struct SystemStats {
    pub sys: System,
    pub disks: sysinfo::Disks,
    pub networks: sysinfo::Networks,
}

impl SystemStats {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        
        let disks = sysinfo::Disks::new_with_refreshed_list();
        let networks = sysinfo::Networks::new_with_refreshed_list();
        Self { sys, disks, networks }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();
        // Fixed for sysinfo 0.33: refresh_processes takes 2 args. 
        // ProcessRefreshKind is usually handled global or per-refresh is deprecated in 0.33? 
        // Let's use refresh_processes which updates all processes CPU/Mem by default in 0.33.
        self.sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        self.disks.refresh(true);
        self.networks.refresh(true);
    }

    pub fn cpu_usage(&self) -> Vec<f32> {
        self.sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
    }

    pub fn mem_usage(&self) -> (u64, u64) {
        (self.sys.used_memory(), self.sys.total_memory())
    }

    pub fn load_avg(&self) -> (f64, f64, f64) {
        let load = System::load_average();
        (load.one, load.five, load.fifteen)
    }

    pub fn cpu_frequencies(&self) -> Vec<u64> {
        self.sys.cpus().iter().map(|cpu| cpu.frequency()).collect()
    }

    pub fn network_io(&self) -> (u64, u64) {
        let mut total_rx = 0;
        let mut total_tx = 0;
        for (_, data) in &self.networks {
            total_rx += data.received();
            total_tx += data.transmitted();
        }
        (total_rx, total_tx)
    }

    pub fn disk_stats(&self) -> Vec<(String, u64, u64)> {
        self.disks.iter().map(|disk| {
            (
                disk.mount_point().to_string_lossy().to_string(),
                disk.total_space() - disk.available_space(),
                disk.total_space()
            )
        }).collect()
    }

    pub fn processes(&self) -> Vec<(u32, String, f32, u64)> {
        self.sys.processes().iter().map(|(pid, proc)| {
            (
                pid.as_u32(),
                proc.name().to_string_lossy().to_string(),
                proc.cpu_usage(),
                proc.memory()
            )
        }).collect()
    }
}
