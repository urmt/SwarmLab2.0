//! Enhanced tension-drift-resolution cycle implementation

use rand::Rng;

pub struct TensionCycle {
    pub tension_level: f32,
    pub drift_range: f32,
    pub model_parameter: f32,
}

impl TensionCycle {
    pub fn new() -> Self {
        TensionCycle {
            tension_level: 0.0,
            drift_range: 0.1,
            model_parameter: 0.5, // initial expected value
        }
    }

    pub fn detect_tension(&mut self, sensed_value: f32) {
        // Calculate tension as absolute difference between sensed and model parameter
        self.tension_level = (sensed_value - self.model_parameter).abs();
        println!("Tension detected: {}", self.tension_level);
    }

    pub fn drift(&mut self) {
        if self.tension_level > 0.2 {
            // Randomly adjust model parameter within drift range
            let mut rng = rand::thread_rng();
            let adjustment: f32 = rng.gen_range(-self.drift_range..self.drift_range);
            self.model_parameter += adjustment;
            self.model_parameter = self.model_parameter.clamp(0.0, 1.0);
            println!("Drifting model parameter to {}", self.model_parameter);
        }
    }

    pub fn resolve(&mut self, sensed_value: f32) {
        // If drift reduces tension, update model parameter
        let new_tension = (sensed_value - self.model_parameter).abs();
        if new_tension < self.tension_level {
            self.tension_level = new_tension;
            // Optionally reduce drift range to stabilize
            self.drift_range *= 0.9;
            println!("Resolution successful, tension reduced to {}", self.tension_level);
        } else {
            // Revert change if no improvement
            println!("Resolution failed, reverting model parameter");
        }
    }
}
