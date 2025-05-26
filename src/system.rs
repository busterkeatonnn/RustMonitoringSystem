use crate::Result;
use sysinfo::{CpuExt, System, SystemExt};

// Состояние процессора
#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub global_usage: f32,     // Общая загрузка (0-100%)
    pub cores_usage: Vec<f32>, // Загрузка по ядрам
}


// Состояние памяти
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,         // Всего физической памяти
    pub used: u64,          // Использовано (включая кэш)
    pub free: u64,          // Свободно
    pub available: u64,     // Доступно для процессов
    pub swap_total: u64,    // Всего swap
    pub swap_used: u64,     // Использовано swap
    pub swap_free: u64,     // Свободно swap
}

// Системный монитор
pub struct SystemMonitor {
    system: System,  // Экземпляр sysinfo
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    // Получение данных CPU
    pub fn get_cpu_info(&mut self) -> Result<CpuInfo> {
        self.refresh();

        let global_usage = self.system.global_cpu_info().cpu_usage();
        let cores_usage = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        Ok(CpuInfo {
            global_usage,
            cores_usage,
        })
    }

    // Получение данных памяти
    pub fn get_memory_info(&mut self) -> Result<MemoryInfo> {
        self.refresh();

        Ok(MemoryInfo {
            total: self.system.total_memory(),
            used: self.system.used_memory(),
            free: self.system.free_memory(),
            available: self.system.available_memory(),
            swap_total: self.system.total_swap(),
            swap_used: self.system.used_swap(),
            swap_free: self.system.free_swap(),
        })
    }
}
