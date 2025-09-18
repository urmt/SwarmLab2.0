//! Core interpreter for WeaveLang

pub mod tension_cycle;
pub mod metaweave;
pub mod parser;

use crate::tension_cycle::TensionCycle;
use crate::metaweave::Metaweave;
use crate::parser::Parser;
use std::collections::HashMap;

pub struct Interpreter {
    tension_cycle: TensionCycle,
    metaweave: Metaweave,
    parser: Parser,
    model_parameter: f32,  // Exposed for wind influence
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            tension_cycle: TensionCycle::new(),
            metaweave: Metaweave::new(),
            parser: Parser::new(),
            model_parameter: 0.5,
        }
    }

    pub fn execute(&mut self, input: &str) {
        let mut lines = input.lines();
        for line in lines {
            if let Some(msg) = self.parser.parse(line) {
                println!("{}", msg);
            } else {
                let c_q = *self.parser.qualic_state.get("C_q").unwrap_or(&0.5);
                let f_q = *self.parser.qualic_state.get("F_q").unwrap_or(&0.7);
                self.tension_cycle.detect_tension(0.5, c_q, f_q);
                self.tension_cycle.drift();
                self.tension_cycle.resolve(0.5, c_q, f_q);
                self.metaweave.propose_new_primitives();
            }
        }
    }

    pub fn should_move_forward(&self) -> bool {
        self.tension_cycle.tension_level > 0.6 && self.model_parameter < 0.5
    }

    pub fn should_move_backward(&self) -> bool {
        self.tension_cycle.tension_level > 0.6 && self.model_parameter > 0.5
    }

    pub fn update_wind_influence(&mut self, wind_speed: f32) {
        // Influence model_parameter with wind_speed (SFH-aligned adjustment)
        self.model_parameter += (wind_speed - 0.5) * 0.1;  // Subtle drift from wind
        self.model_parameter = self.model_parameter.clamp(0.0, 1.0);
        println!("Wind influence updated model_parameter to {}", self.model_parameter);
    }
}
