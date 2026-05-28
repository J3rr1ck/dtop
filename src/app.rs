use crate::stats::SystemStats;
use crate::gpu::GpuStats;
use crate::theme::{Theme, ThemePreset};
use std::collections::VecDeque;

pub struct App {
    pub system_stats: SystemStats,
    pub gpu_stats: GpuStats,
    pub theme: Theme,
    
    pub cpu_history: VecDeque<f32>,
    pub gpu_load_history: VecDeque<f32>,
    pub gpu_mem_history: VecDeque<f32>,
    pub gpu_temp_history: VecDeque<f32>,
    pub gpu_power_history: VecDeque<f32>,
    pub net_rx_history: VecDeque<f32>,
    pub net_tx_history: VecDeque<f32>,
    
    pub selected_gpu: usize,
    pub history_limit: usize,
    
    pub show_theme_menu: bool,
    pub theme_index: usize,
    pub themes: Vec<ThemePreset>,
}

impl App {
    pub fn new() -> App {
        let limit = 100;
        let themes = vec![
            ThemePreset::DeepSpace,
            ThemePreset::Nord,
            ThemePreset::HighContrast,
            ThemePreset::SolarizedDark,
            ThemePreset::SolarizedLight,
            ThemePreset::GruvboxDark,
            ThemePreset::GruvboxLight,
            ThemePreset::Monokai,
            ThemePreset::OneDark,
            ThemePreset::Dracula,
            ThemePreset::TokyoNight,
            ThemePreset::Catppuccin,
            ThemePreset::Cyberpunk,
        ];
        App {
            system_stats: SystemStats::new(),
            gpu_stats: GpuStats::new(),
            theme: Theme::from_preset(themes[0]),
            cpu_history: VecDeque::from(vec![0.0; limit]),
            gpu_load_history: VecDeque::from(vec![0.0; limit]),
            gpu_mem_history: VecDeque::from(vec![0.0; limit]),
            gpu_temp_history: VecDeque::from(vec![0.0; limit]),
            gpu_power_history: VecDeque::from(vec![0.0; limit]),
            net_rx_history: VecDeque::from(vec![0.0; limit]),
            net_tx_history: VecDeque::from(vec![0.0; limit]),
            selected_gpu: 0,
            history_limit: limit,
            show_theme_menu: false,
            theme_index: 0,
            themes,
        }
    }

    pub fn on_tick(&mut self) {
        self.system_stats.refresh();
        self.gpu_stats.refresh();

        let cpus = self.system_stats.cpu_usage();
        let avg_cpu = if cpus.is_empty() { 0.0 } else { cpus.iter().sum::<f32>() / cpus.len() as f32 };
        Self::push_history(&mut self.cpu_history, avg_cpu, self.history_limit);

        let (rx, tx) = self.system_stats.network_io();
        Self::push_history(&mut self.net_rx_history, rx as f32, self.history_limit);
        Self::push_history(&mut self.net_tx_history, tx as f32, self.history_limit);

        let gpus = self.gpu_stats.get_gpus();
        if !gpus.is_empty() {
            let gpu = if self.selected_gpu < gpus.len() { &gpus[self.selected_gpu] } else { &gpus[0] };
            Self::push_history(&mut self.gpu_load_history, gpu.load as f32, self.history_limit);
            let mem_percent = if gpu.mem_total == 0 { 0.0 } else { gpu.mem_used as f32 * 100.0 / gpu.mem_total as f32 };
            Self::push_history(&mut self.gpu_mem_history, mem_percent, self.history_limit);
            Self::push_history(&mut self.gpu_temp_history, gpu.temp as f32, self.history_limit);
            let power_percent = if gpu.power_limit == 0 { 0.0 } else { gpu.power_usage as f32 * 100.0 / gpu.power_limit as f32 };
            Self::push_history(&mut self.gpu_power_history, power_percent, self.history_limit);
        } else {
            Self::push_history(&mut self.gpu_load_history, 0.0, self.history_limit);
            Self::push_history(&mut self.gpu_mem_history, 0.0, self.history_limit);
            Self::push_history(&mut self.gpu_temp_history, 0.0, self.history_limit);
            Self::push_history(&mut self.gpu_power_history, 0.0, self.history_limit);
        }
    }

    fn push_history(history: &mut VecDeque<f32>, val: f32, limit: usize) {
        history.push_back(val);
        if history.len() > limit {
            history.pop_front();
        }
    }

    pub fn next_gpu(&mut self) {
        let count = self.gpu_stats.get_gpus().len();
        if count > 0 {
            self.selected_gpu = (self.selected_gpu + 1) % count;
            self.gpu_load_history = VecDeque::from(vec![0.0; self.history_limit]);
            self.gpu_mem_history = VecDeque::from(vec![0.0; self.history_limit]);
            self.gpu_temp_history = VecDeque::from(vec![0.0; self.history_limit]);
            self.gpu_power_history = VecDeque::from(vec![0.0; self.history_limit]);
        }
    }

    pub fn toggle_theme_menu(&mut self) {
        self.show_theme_menu = !self.show_theme_menu;
    }

    pub fn next_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % self.themes.len();
        self.theme = Theme::from_preset(self.themes[self.theme_index]);
    }

    pub fn prev_theme(&mut self) {
        if self.theme_index == 0 {
            self.theme_index = self.themes.len() - 1;
        } else {
            self.theme_index -= 1;
        }
        self.theme = Theme::from_preset(self.themes[self.theme_index]);
    }
}
