use sysmonitor::{NetworkInfo, Process, ProcessFilter, SystemMonitor};

fn main() {
    let mut process_monitor = Process::new();

    let all_processes = process_monitor.get_all_processes();
    println!("Количество процессов: {}", all_processes.len());
    println!("Первые 10 процессов");
    let mut i = 10;
    println!("\t{:20} / {:40} / {:20}", "pid", "name", "cpu_usage");
    println!("\t{:-<1$}", "", 20 + 40 + 20 + 1);
    for (key, value) in all_processes.into_iter() {
        println!(
            "\t{:<20} / {:40} / {:<20}",
            key, value.name, value.cpu_usage
        );

        if i == 0 {
            break;
        }
        i = i - 1;
    }

    if let Ok(process) = process_monitor.get_process_by_pid(3170) {
        println!("\nИнформация по процессу с PID = 3170");
        println!("\tPID: {}", process.pid);
        println!("\tНазвание: {}", process.name);
        println!("\tID пользователя: {}", process.user);
        println!("\tИспользование CPU (%): {}", process.cpu_usage);
        println!(
            "\tИсопльзование памяти (Мб.): {}",
            process.memory / (1024 * 1024)
        );
        println!("\tВремя работы: {}", process.start_time);
    }


    let heavy_processes_ram =
        process_monitor.find_processes_by_filter(ProcessFilter::MemoryMoreThan(512 * 1024 * 1024));
    println!(
        "\nКоличество процессов, использующих >512МБ: {}",
        heavy_processes_ram.len()
    );
    for (key, value) in heavy_processes_ram.into_iter() {
        println!("\t{} / {}", key, value.name);
    }

    let heavy_processes_cpu =
        process_monitor.find_processes_by_filter(ProcessFilter::CpuMoreThan(10.0));
    println!(
        "\nКоличество процессов, использующих >10% CPU: {}",
        heavy_processes_cpu.len()
    );
    for (key, value) in heavy_processes_cpu.into_iter() {
        println!("\t{} / {}", key, value.name);
    }

    let mut sys_monitor = SystemMonitor::new();

    if let Ok(cpu_info) = sys_monitor.get_cpu_info() {
        println!("\nОбщая загрузка CPU: {}%", cpu_info.global_usage);
        println!("Загрузка по ядрам: {:?}", cpu_info.cores_usage);
    }

    if let Ok(mem_info) = sys_monitor.get_memory_info() {
        println!(
            "\nПамять: используется {} из {} МБ",
            mem_info.used / (1024 * 1024),
            mem_info.total / (1024 * 1024)
        );
    }

    let mut net_monitor = NetworkInfo::new();

    if let Ok(net_metrics) = net_monitor.get_network_metrics() {
        for (key, value) in &net_metrics {
            println!("\nИнтерфейс '{}'", key);
            println!("\tВсего принято байт: {}", value.total_received_bytes);
            println!("\tВсего отправлено байт: {}", value.total_transmitted_bytes);
            println!("\tВсего получено пакетов: {}", value.total_received_packets);
            println!(
                "\tВсего отправено пакетов: {}",
                value.total_transmitted_packets
            );
            println!(
                "\tКоличество ошибок при принятии: {}",
                value.errors_on_received
            );
            println!(
                "\tКоличество ошибок при передаче {}",
                value.errors_on_transmitted
            );
        }
    }

    if let Ok(net_throughput) = net_monitor.get_network_throughput(1000) {
        for (key, value) in &net_throughput {
            println!("\nИнтерфейс '{}'", key);
            println!("\tDownlink: {}", value.0);
            println!("\tUplink: {}", value.1);
        }
    }
}
