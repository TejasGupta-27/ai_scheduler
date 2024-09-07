use std::collections::HashMap;
use rand::Rng;

pub struct QLearningAgent {
    q_table: HashMap<(u64, u64), f64>,  // State-action pairs
    alpha: f64,  // Learning rate
    gamma: f64,  // Discount factor
}

impl QLearningAgent {
    pub fn new() -> Self {
        QLearningAgent {
            q_table: HashMap::new(),
            alpha: 0.1,
            gamma: 0.9,
        }
    }

    pub fn choose_action(&self, state: u64) -> u64 {
        if rand::thread_rng().gen_range(0.0..1.0) < 0.1 {
            // Explore: Random action
            rand::thread_rng().gen_range(0..5)
        } else {
            // Exploit: Choose the action with max Q-value
            self.q_table.iter()
                .filter(|&(&(s, _), _)| s == state)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(&(_s, a), _)| a)
                .unwrap_or(0)
        }
    }

    pub fn update_q_value(&mut self, state: u64, action: u64, reward: f64, next_state: u64) {
        let max_next_q = *self.q_table.get(&(next_state, action)).unwrap_or(&0.0);
        let current_q = *self.q_table.get(&(state, action)).unwrap_or(&0.0);
        let new_q = current_q + self.alpha * (reward + self.gamma * max_next_q - current_q);
        self.q_table.insert((state, action), new_q);
    }
}
