//! Enhanced tension-drift-resolution cycle implementation with SFH alignment

use rand::Rng;
use std::f32::consts::E;

pub struct TensionCycle {
    pub tension_level: f32,
    pub drift_range: f32,
    pub model_parameter: f32,
    pub alpha: f32, // Coherence weight
    pub beta: f32,  // Fertility weight
}

impl TensionCycle {
    pub fn new() -> Self {
        TensionCycle {
            tension_level: 0.0,
            drift_range: 0.1,
            model_parameter: 0.5,
            alpha: 0.5, // Default weights
            beta: 0.5,
        }
    }

    pub fn detect_tension(&mut self, sensed_value: f32, c_q: f32, f_q: f32) {
        // Tension as weighted difference via J(q) = αC(q) + βF(q)
        let j_q = self.alpha * c_q + self.beta * f_q;
        self.tension_level = (sensed_value - j_q).abs();
        println!("Tension detected: {}", self.tension_level);
    }

    pub fn drift(&mut self) {
        if self.tension_level > 0.2 {
            let mut rng = rand::thread_rng();
            let noise = rng.gen_range(-self.drift_range..self.drift_range) * (1.0 / (self.tension_level * E).exp());
            self.model_parameter += noise;
            self.model_parameter = self.model_parameter.clamp(0.0, 1.0);
            println!("Drifting model parameter to {} with noise {}", self.model_parameter, noise);
        }
    }

    pub fn resolve(&mut self, sensed_value: f32, c_q: f32, f_q: f32) {
        let j_q = self.alpha * c_q + self.beta * f_q;
        let new_tension = (sensed_value - j_q).abs();
        if new_tension < self.tension_level {
            self.tension_level = new_tension;
            self.drift_range *= 0.9; // Stabilize
            println!("Resolution successful, tension reduced to {}", self.tension_level);
        } else {
            self.model_parameter -= self.model_parameter * 0.1; // Gradient-like adjustment
            println!("Resolution failed, adjusting model parameter to {}", self.model_parameter);
        }
    }
}
