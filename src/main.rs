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

    // Handle subcommands from the user
    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            info!("Starting AI Process Scheduler...");

            // Initialize and train the RL agent
            rl_agent::initialize_agent().await;
            rl_agent::train_agent().await;

            // Monitor system metrics
            monitor::start_monitoring().await;

            // Start the AI scheduling system
            scheduler::start_scheduling().await;
        }
        Some(("status", _)) => {
            info!("Displaying system status...");
            monitor::display_system_status();
        }
        _ => {
            eprintln!("No valid subcommand provided.");
        }
    }
}
