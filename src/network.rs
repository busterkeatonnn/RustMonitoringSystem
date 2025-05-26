use crate::Result;
use std::collections::HashMap;
use sysinfo::{NetworkExt, System, SystemExt};

// Структура для хранения сетевой статистики интерфейса
#[derive(Debug, Clone)]
pub struct NetworkInterfaceInfo {
    pub total_received_bytes: u64, // Всего получено за всё время
    pub received_bytes: u64,  // Получено с последнего обновления
    pub total_transmitted_bytes: u64, // Всего отправлено за всё время
    pub transmitted_bytes: u64, // Отправлено с последнего обновления
    pub total_received_packets: u64, // Всего пакетов
    pub received_packets: u64,  // Пакетов с последнего обновления
    pub total_transmitted_packets: u64, // Всего пакетов отправки
    pub transmitted_packets: u64, // Пакетов отправки с обновления
    pub errors_on_received: u64, // Ошибок при приёме
    pub errors_on_transmitted: u64,  // Ошибок при отправке
}

// Основной класс для работы с сетью
pub struct NetworkInfo {
    system: System,  // Экземпляр sysinfo
    previous_stats: HashMap<String, NetworkInterfaceInfo>, // Предыдущие замеры
}

impl NetworkInfo {
    // Создание нового монитора с полным обновлением данных
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();  // Первоначальное обновление всех данных
        Self {
            system,
            previous_stats: HashMap::new(),
        }
    }

    // Метод для обновления сетевых данных
    pub fn refresh(&mut self) {
        self.system.refresh_networks(); // Точечное обновление сети
    }

    // Метод получения сетевых метрик
    pub fn get_network_metrics(&mut self) -> Result<HashMap<String, NetworkInterfaceInfo>> {
        self.refresh(); // Гарантия актуальных данных

        let mut result = HashMap::new();

        // Итерация по сетевым интерфейсам
        for (interface_name, data) in self.system.networks() {
            let info = NetworkInterfaceInfo {  // Заполнение всех полей
                total_received_bytes: data.total_received(),
                received_bytes: data.received(),
                total_transmitted_bytes: data.total_transmitted(),
                transmitted_bytes: data.transmitted(),
                total_received_packets: data.total_packets_received(),
                received_packets: data.packets_received(),
                total_transmitted_packets: data.total_packets_transmitted(),
                transmitted_packets: data.packets_transmitted(),
                errors_on_received: data.errors_on_received(),
                errors_on_transmitted: data.errors_on_transmitted(),
            };

            result.insert(interface_name.to_string(), info);
        }

        Ok(result)
    }

    // Расчет пропускной способности
    pub fn get_network_throughput(
        &mut self,
        interval_ms: u64, // Интервал измерения в миллисекундах
    ) -> Result<HashMap<String, (f64, f64)>> {
        let initial_metrics = self.get_network_metrics()?;

        for (name, info) in &initial_metrics {
            self.previous_stats.insert(name.clone(), info.clone());
        }

        // Задержка для измерения
        std::thread::sleep(std::time::Duration::from_millis(interval_ms));

        let new_metrics = self.get_network_metrics()?;

        let mut throughput = HashMap::new();

        // Расчет разницы между замерами
        for (name, info) in new_metrics {
            if let Some(prev_info) = self.previous_stats.get(&name) {
                let rx_bytes_per_sec = (info.received_bytes as f64
                    - prev_info.received_bytes as f64)
                    / (interval_ms as f64 / 1000.0);
                let tx_bytes_per_sec = (info.transmitted_bytes as f64
                    - prev_info.transmitted_bytes as f64)
                    / (interval_ms as f64 / 1000.0);

                throughput.insert(name, (rx_bytes_per_sec, tx_bytes_per_sec));
            }
        }

        Ok(throughput)
    }
}
