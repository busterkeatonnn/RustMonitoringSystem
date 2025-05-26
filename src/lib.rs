// Объявление внутренних модулей
mod network;    // Сетевые метрики
mod process;    // Управление процессами
mod system;     // Системные ресурсы

// Публичный интерфейс библиотеки
pub use network::NetworkInfo;  // Экспорт сетевого функционала
pub use process::{Process, ProcessFilter, ProcessInfo};  // Экспорт процессов
pub use system::{CpuInfo, MemoryInfo, SystemMonitor};  // Экспорт системных данных

// Интеграция с системой обработки ошибок
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SysMonitorError {
    #[error("Ошибка получения данных о процессе: {0}")]
    ProcessError(String),

    #[error("Ошибка получения системных метрик: {0}")]
    SystemError(String),

    #[error("Ошибка получения сетевых метрик: {0}")]
    NetworkError(String),

    #[error("Процесс не найден: {0}")]
    ProcessNotFound(u32),
}

// Удобный псевдоним для Result
pub type Result<T> = std::result::Result<T, SysMonitorError>;

