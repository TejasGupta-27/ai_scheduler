use crate::rl_agent::QLearningAgent;
use log::info;
use sysinfo::{System, SystemExt, ProcessorExt};

pub async fn start_scheduling() {
    let mut agent = QLearningAgent::new();

    loop {
        // Monitor system metrics and get the current state
        let system_state = get_system_state().await;

        // Agent chooses an action (process scheduling decision)
        let action = agent.choose_action(system_state);

        // Take the action (e.g., adjust process priorities/resources)
        take_action(action).await;

        // Calculate reward (e.g., system throughput, efficiency, reduced wait times)
        let reward = calculate_reward().await;

        // Update the RL agent with the reward and next state
        let next_state = get_system_state().await;
        agent.update_q_value(system_state, action, reward, next_state);
    
        // Continue scheduling in a loop
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
}

// Fetch system state
async fn get_system_state() -> u64 {
    let system = System::new_all();
    // system.refresh_all();
    let cpu_usage = system.global_processor_info().cpu_usage();
    let total_memory = system.total_memory();
    let used_memory = total_memory - system.free_memory();

    // Create a unique state representation
    (cpu_usage as u64) * 1000000 + (used_memory / 1024)
}

// Take action
async fn take_action(action: u64) {
    info!("Taking action: {}", action);
    
    // Simulate adjusting process priority or resource allocation
    match action % 3 {
        0 => info!("Increasing CPU priority for critical tasks"),
        1 => info!("Decreasing CPU priority for non-critical tasks"),
        2 => info!("Adjusting memory allocation for tasks"),
        _ => info!("Unknown action"),
    }
}

// Calculate reward
async fn calculate_reward() -> f64 {
    let system = System::new_all();
    let cpu_usage = system.global_processor_info().cpu_usage();
    let total_memory = system.total_memory();
    let free_memory = system.free_memory();
    let used_memory = total_memory - free_memory;

    // Reward calculation example:
    // Reward is inversely proportional to CPU usage and memory usage
    100.0 - (cpu_usage as f64 + (used_memory / 1024) as f64)
}