mod scheduler;
mod rl_agent;
mod monitor;
mod cli;
mod utils;

use log::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Setup command line arguments
    let matches = cli::setup_cli();

    // Handle commands from the user
    if matches.get_flag("start") {
        info!("Starting AI Process Scheduler...");

        // Monitor system metrics
        monitor::start_monitoring().await;

        // Start the AI scheduling system
        scheduler::start_scheduling().await;

    } else if matches.get_flag("status") {
        info!("Displaying system status...");
        monitor::display_system_status();
    }
}
