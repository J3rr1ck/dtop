use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::{TemperatureSensor, Clock};
use nvml_wrapper::enums::device::UsedGpuMemory;

pub struct GpuProcessInfo {
    pub pid: u32,
    pub used_mem: u64,
}

pub struct GpuInfo {
    pub name: String,
    pub load: u32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub temp: u32,
    pub fan_speed: u32,
    pub power_usage: u32, // in milliwatts
    pub power_limit: u32, // in milliwatts
    pub graphics_clock: u32, // in MHz
    pub memory_clock: u32, // in MHz
    pub processes: Vec<GpuProcessInfo>,
}

pub struct GpuStats {
    nvml: Option<Nvml>,
}

impl GpuStats {
    pub fn new() -> Self {
        let nvml = Nvml::init().ok();
        Self { nvml }
    }

    pub fn refresh(&mut self) {
        // NVML handles refresh on query mostly
    }

    pub fn get_gpus(&self) -> Vec<GpuInfo> {
        let mut gpus = Vec::new();
        if let Some(nvml) = &self.nvml {
            let device_count = nvml.device_count().unwrap_or(0);
            for i in 0..device_count {
                if let Ok(device) = nvml.device_by_index(i) {
                    let name = device.name().unwrap_or_else(|_| "Unknown".to_string());
                    let util = device.utilization_rates().ok();
                    let mem = device.memory_info().ok();
                    let temp = device.temperature(TemperatureSensor::Gpu).unwrap_or(0);
                    let fan = device.fan_speed(0).unwrap_or(0); // Index 0 for fan
                    let power = device.power_usage().unwrap_or(0);
                    let power_limit = device.enforced_power_limit().unwrap_or(0);
                    let g_clock = device.clock_info(Clock::Graphics).unwrap_or(0);
                    let m_clock = device.clock_info(Clock::Memory).unwrap_or(0);

                    let mut gpu_processes = Vec::new();
                    if let Ok(procs) = device.running_compute_processes() {
                        for p in procs {
                            let mem = match p.used_gpu_memory {
                                UsedGpuMemory::Used(m) => m,
                                UsedGpuMemory::Unavailable => 0,
                            };
                            gpu_processes.push(GpuProcessInfo {
                                pid: p.pid,
                                used_mem: mem,
                            });
                        }
                    }

                    gpus.push(GpuInfo {
                        name,
                        load: util.as_ref().map(|u| u.gpu).unwrap_or(0),
                        mem_used: mem.as_ref().map(|m| m.used).unwrap_or(0),
                        mem_total: mem.as_ref().map(|m| m.total).unwrap_or(0),
                        temp,
                        fan_speed: fan,
                        power_usage: power,
                        power_limit,
                        graphics_clock: g_clock,
                        memory_clock: m_clock,
                        processes: gpu_processes,
                    });
                }
            }
        }
        gpus
    }
}
