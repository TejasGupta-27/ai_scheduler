mod scheduler;
mod rl_agent;
mod monitor;
mod cli;
mod utils;

use cli::setup_cli;
use log::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Setup command line arguments
    let matches = setup_cli();

    // Handle commands from the user
    if matches.contains_id("start") {
        info!("Starting AI Process Scheduler...");

        // Monitor system metrics
        monitor::start_monitoring().await;

        // Start the AI scheduling system
        scheduler::start_scheduling().await;

    } else if matches.contains_id("status") {
        info!("Displaying system status...");
        monitor::display_system_status();
    }
}
