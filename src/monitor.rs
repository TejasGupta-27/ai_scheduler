// src/monitor.rs
use log::info;
use sysinfo::{System, SystemExt, ProcessorExt};

pub async fn start_monitoring() {
    info!("System monitoring started");
    tokio::spawn(async move {
        loop {
            display_system_status();
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
}

pub fn display_system_status() {
    let mut system = System::new_all();
    system.refresh_all();
    
    let cpu_usage = system.global_processor_info().cpu_usage();
    let total_memory = system.total_memory();
    let used_memory = total_memory - system.free_memory();
    
    info!("System Status:");
    info!("CPU Usage: {:.2}%", cpu_usage);
    info!("Memory Usage: {}/{} MB", used_memory / 1024 / 1024, total_memory / 1024 / 1024);
}