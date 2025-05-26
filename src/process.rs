use crate::{Result, SysMonitorError};
use std::collections::HashMap;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

// Детальная информация о процессе
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,          // Уникальный идентификатор
    pub name: String,      // Имя исполняемого файла
    pub cmd: Vec<String>,  // Командная строка запуска
    pub memory: u64,       // Потребление RAM в байтах
    pub cpu_usage: f32,    // Процент использования CPU
    pub start_time: u64,   // Временная метка запуска
    pub user: String,      // Владелец процесса (UID)
}

// Фильтры для поиска процессов
pub enum ProcessFilter {
    MemoryLessThan(u64),  // Верхняя граница памяти
    MemoryMoreThan(u64),  // Нижняя граница памяти
    CpuLessThan(f32),     // Верхняя граница CPU
    CpuMoreThan(f32),     // Нижняя граница CPU
}

// Основная структура для работы с процессами
pub struct Process {
    system: System, // Экземпляр sysinfo
}

impl Process {
    // Инициализация с полным сканированием
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    // Получение всех процессов
    pub fn get_all_processes(&mut self) -> HashMap<u32, ProcessInfo> {
        self.refresh();

        self.system
            .processes()
            .iter()
            .map(|(pid, proc)| {
                let pid_u32 = pid.as_u32();
                let process_info = ProcessInfo {
                    pid: pid_u32,
                    name: proc.name().to_string(),
                    cmd: proc.cmd().to_vec(),
                    memory: proc.memory(),
                    cpu_usage: proc.cpu_usage(),
                    start_time: proc.start_time(),
                    user: proc
                        .user_id()
                        .map(|uid| uid.to_string())
                        .unwrap_or_else(|| "unknown".to_string()),
                };
                (pid_u32, process_info)
            })
            .collect()
    }

    // Поиск процесса по PID
    pub fn get_process_by_pid(&mut self, pid: u32) -> Result<ProcessInfo> {
        self.refresh();

        if let Some(process) = self.system.process(sysinfo::Pid::from(pid as usize)) {
            Ok(ProcessInfo {
                pid,
                name: process.name().to_string(),
                cmd: process.cmd().to_vec(),
                memory: process.memory(),
                cpu_usage: process.cpu_usage(),
                start_time: process.start_time(),
                user: process
                    .user_id()
                    .map(|uid| uid.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
            })
        } else {
            Err(SysMonitorError::ProcessNotFound(pid))
        }
    }

    // Фильтрация процессов
    pub fn find_processes_by_filter(
        &mut self,
        filter: ProcessFilter  // Применяемый фильтр
    ) -> HashMap<u32, ProcessInfo> {
        let all_processes = self.get_all_processes();

        match filter {
            ProcessFilter::MemoryLessThan(limit) => all_processes
                .into_iter()
                .filter(|(_, info)| info.memory < limit)
                .collect(),
            ProcessFilter::MemoryMoreThan(limit) => all_processes
                .into_iter()
                .filter(|(_, info)| info.memory > limit)
                .collect(),
            ProcessFilter::CpuLessThan(limit) => all_processes
                .into_iter()
                .filter(|(_, info)| info.cpu_usage < limit)
                .collect(),
            ProcessFilter::CpuMoreThan(limit) => all_processes
                .into_iter()
                .filter(|(_, info)| info.cpu_usage > limit)
                .collect(),
        }
    }
}
