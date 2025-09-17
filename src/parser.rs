// parser.rs
//! WeaveLang parser module for handling qualic commands and gradient computation

use std::collections::HashMap;

pub struct Parser {
    qualic_state: HashMap<String, f32>,
    epsilon: f32, // Step size for finite differences
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            qualic_state: HashMap::new(),
            epsilon: 1e-6, // Small step for gradient approximation
        }
    }

    pub fn parse(&mut self, line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.get(0).copied() {
            Some("qualic") => self.parse_qualic(&parts[1..]),
            Some("partition") => self.parse_partition(&parts),
            _ => None,
        }
    }

    fn parse_qualic(&mut self, parts: &[&str]) -> Option<String> {
        if parts.len() >= 3 && parts[0] == "set" {
            if let Ok(value) = parts[2].parse::<f32>() {
                self.qualic_state.insert(parts[1].to_string(), value);
                Some(format!("Set qualic state {} = {}", parts[1], value))
            } else {
                Some("Invalid qualic value".to_string())
            }
        } else {
            None
        }
    }

    fn parse_partition(&mut self, parts: &[&str]) -> Option<String> {
        if parts.len() >= 2 && parts[1].starts_with("kmax=") {
            if let Ok(kmax) = parts[1].split('=').nth(1).unwrap_or("5").parse::<i32>() {
                // Placeholder for partition effect (e.g., update Metaweave)
                Some(format!("Partition set with kmax = {}", kmax))
            } else {
                Some("Invalid kmax value".to_string())
            }
        } else {
            None
        }
    }

    pub fn compute_gradient(&self, j_q: f32, param: f32) -> f32 {
        // Finite difference approximation of gradient
        let delta = self.epsilon;
        let j_q_plus = self.evaluate_j_q(param + delta); // Assume j_q is a function of param
        let j_q_minus = self.evaluate_j_q(param - delta);
        (j_q_plus - j_q_minus) / (2.0 * delta)
    }

    fn evaluate_j_q(&self, param: f32) -> f32 {
        // Placeholder J(q) evaluation based on qualic_state
        let c_q = self.qualic_state.get("C_q").unwrap_or(&0.5);
        let f_q = self.qualic_state.get("F_q").unwrap_or(&0.7);
        let alpha = 0.5; // Default weights
        let beta = 0.5;
        alpha * c_q + beta * f_q + param // Simplified, param as a variable term
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_qualic() {
        let mut parser = Parser::new();
        let result = parser.parse("qualic set C_q = 0.8").unwrap();
        assert_eq!(result, "Set qualic state C_q = 0.8");
        assert_eq!(*parser.qualic_state.get("C_q").unwrap(), 0.8);
    }

    #[test]
    fn test_compute_gradient() {
        let parser = Parser::new();
        let gradient = parser.compute_gradient(1.0, 0.5);
        assert!(!gradient.is_nan()); // Ensure valid result
        assert!(gradient.abs() < 1.0); // Reasonable gradient magnitude
    }
}
