use ndarray::{Array2, ArrayView2};
use rand::distributions::{Distribution, Normal};
use rand::thread_rng;
use crate::qualic_partition::{compute_partition_peaks, discretize_to_peaks};

pub struct TensionCycle {
    weights: Array2<f64>,
    noise_dist: Normal,
    alpha: f64,
    beta: f64,
    peaks: Array1<f64>, // Added for partition peaks
    convergence_threshold: f64, // New for dynamic stopping
}

impl TensionCycle {
    pub fn new(agent_count: usize, sensor_dim: usize, discretize: bool) -> Self {
        let weights = Array2::from_elem((agent_count, sensor_dim), 0.7);
        let noise_dist = Normal::new(0.5, 0.2);
        let peaks = if discretize {
            compute_partition_peaks(30)
        } else {
            Array1::zeros(1) // Dummy for continuous mode
        };
        TensionCycle {
            weights,
            noise_dist,
            alpha: 0.6,
            beta: 0.4,
            peaks,
            convergence_threshold: 0.01,
        }
    }

    pub fn tension(&self, sensors: &Array2<f64>) -> Array2<f64> {
        sensors.dot(&self.weights.t()) // C_q = sum w_i s_i
    }

    pub fn drift(&self, c_q: &Array2<f64>) -> Array2<f64> {
        let mut rng = thread_rng();
        let noise = Array2::from_shape_fn(c_q.dim(), |_| self.noise_dist.sample(&mut rng));
        let f_q = c_q + noise;
        if self.peaks.len() > 1 {
            discretize_to_peaks(&f_q, &self.peaks)
        } else {
            f_q
        }
    }

    pub fn resolution(&self, c_q: &Array2<f64>, f_q: &Array2<f64>) -> f64 {
        let c_score = c_q.mean().unwrap_or(0.0);
        let f_score = f_q.std(0.0);
        self.alpha * c_score + self.beta * f_score // J(q) = αC + βF
    }

    pub fn cycle(&self, sensors: ArrayView2<f64>) -> (Array2<f64>, Array2<f64>, f64, usize) {
        let mut j_prev = 0.0;
        let mut iterations = 0;
        let c_q = self.tension(&sensors.to_owned());
        let mut f_q = self.drift(&c_q);
        let mut j_q = self.resolution(&c_q, &f_q);
        while (j_q - j_prev).abs() > self.convergence_threshold && iterations < 100 {
            j_prev = j_q;
            f_q = self.drift(&c_q);
            j_q = self.resolution(&c_q, &f_q);
            iterations += 1;
        }
        (c_q, f_q, j_q, iterations)
    }
}
