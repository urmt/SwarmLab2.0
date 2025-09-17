//! Core interpreter for WeaveLang

pub mod tension_cycle;
pub mod metaweave;

use crate::tension_cycle::TensionCycle;
use crate::metaweave::Metaweave;
use std::collections::HashMap;

pub struct Interpreter {
    tension_cycle: TensionCycle,
    metaweave: Metaweave,
    qualic_state: HashMap<String, f32>, // Track qualic parameters
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            tension_cycle: TensionCycle::new(),
            metaweave: Metaweave::new(),
            qualic_state: HashMap::new(),
        }
    }

    pub fn should_move_forward(&self) -> bool {
        self.tension_cycle.tension_level > 0.6 && self.tension_cycle.model_parameter < 0.5
    }

    pub fn should_move_backward(&self) -> bool {
        self.tension_cycle.tension_level > 0.6 && self.tension_cycle.model_parameter > 0.5
        }
    }
    
    pub fn execute(&mut self, input: &str) {
        // Parse WeaveLang code (simplified parser)
        let mut lines = input.lines();
        for line in lines {
            if line.starts_with("qualic ") {
                self.parse_qualic(line);
            } else {
                self.tension_cycle.detect_tension(0.5); // Default sensed value
                self.tension_cycle.drift();
                self.tension_cycle.resolve(0.5);
                self.metaweave.propose_new_primitives();
            }
        }
    }

    fn parse_qualic(&mut self, line: &str) {
        // Example: "qualic set C_q = 0.8" or "qualic partition kmax=10"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 && parts[1] == "set" {
            if let Ok(value) = parts[3].parse::<f32>() {
                self.qualic_state.insert(parts[2].to_string(), value);
                println!("Set qualic state {} = {}", parts[2], value);
            }
        } else if parts.len() >= 3 && parts[1] == "partition" {
            if let Some(kmax) = parts[2].split('=').nth(1).and_then(|s| s.parse::<i32>().ok()) {
                self.metaweave.update_partitions(kmax);
            }
        }
    }
}
