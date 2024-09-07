use procfs::process::all_processes;
use log::info;

pub async fn start_monitoring() {
   
    let all_procs = all_processes().unwrap();
    for proc in all_procs {
        let stat = proc.unwrap().stat().unwrap();
        info!("PID: {}, CPU: {}, Memory: {}", stat.pid, stat.utime, stat.rss);
    }
}

pub fn display_system_status() {
   
    // Display a summary of system status (CPU, memory, and process load)
    // Implement as needed for status output
}
