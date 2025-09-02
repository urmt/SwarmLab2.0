//! Core interpreter for WeaveLang

pub mod tension_cycle;
pub mod metaweave;

use crate::tension_cycle::TensionCycle;
use crate::metaweave::Metaweave;

pub struct Interpreter {
    tension_cycle: TensionCycle,
    metaweave: Metaweave,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            tension_cycle: TensionCycle::new(),
            metaweave: Metaweave::new(),
        }
    }

    pub fn execute(&mut self, input: &str) {
        // Parse and execute WeaveLang code (simplified)
        // For now, just simulate tension-drift-resolution cycle
        self.tension_cycle.detect_tension();
        self.tension_cycle.drift();
        self.tension_cycle.resolve();

        // Metaweave dynamic extension
        self.metaweave.propose_new_primitives();
    }
}
