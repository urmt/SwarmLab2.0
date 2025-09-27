#[cfg(test)]
mod tests {
    use crate::tension_cycle::TensionCycle;
    use crate::qualic_partition::{compute_partition_peaks, discretize_to_peaks};
    use ndarray::{array, Array2};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_tension_phase() {
        let cycle = TensionCycle::new(2, 3, false); // 2 agents, 3D sensors, no discretization
        let sensors = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let c_q = cycle.tension(&sensors);
        assert_eq!(c_q.shape(), [2, 2]); // Output shape: agents x agents
        assert_abs_diff_eq!(c_q[[0, 0]], 1.4 * 1.0 + 2.0 * 0.7 + 3.0 * 0.7, epsilon = 1e-10); // Weighted sum
    }

    #[test]
    fn test_drift_phase_continuous() {
        let cycle = TensionCycle::new(2, 3, false); // No discretization
        let c_q = array![[1.0, 2.0], [3.0, 4.0]];
        let f_q = cycle.drift(&c_q);
        assert_eq!(f_q.shape(), [2, 2]);
        // Check that drift adds noise (approximate due to randomness)
        assert!(f_q.iter().any(|&x| (x - 0.5).abs() < 0.5)); // Within noise range (mean 0.5, std 0.2)
    }

    #[test]
    fn test_drift_phase_discretized() {
        let cycle = TensionCycle::new(2, 3, true); // With discretization
        let c_q = array![[0.1, 0.4], [0.7, 0.2]];
        let f_q = cycle.drift(&c_q);
        let peaks = compute_partition_peaks(30);
        assert_eq!(f_q.shape(), [2, 2]);
        // Verify all values are snapped to nearest peak
        assert!(f_q.iter().all(|&x| peaks.iter().any(|&p| (x - p).abs() < 1e-10)));
    }

    #[test]
    fn test_resolution_phase() {
        let cycle = TensionCycle::new(2, 3, false);
        let c_q = array![[1.0, 2.0], [3.0, 4.0]];
        let f_q = array![[1.5, 2.5], [3.5, 4.5]];
        let j_q = cycle.resolution(&c_q, &f_q);
        let expected = 0.6 * (1.0 + 2.0 + 3.0 + 4.0) / 4.0 + 0.4 * ((1.5 - 3.0).powi(2) + (2.5 - 3.0).powi(2) + (3.5 - 3.0).powi(2) + (4.5 - 3.0).powi(2)).sqrt() / 2.0;
        assert_abs_diff_eq!(j_q, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_full_cycle() {
        let cycle = TensionCycle::new(2, 3, true); // With discretization
        let sensors = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let (c_q, f_q, j_q) = cycle.cycle(sensors.view());
        assert_eq!(c_q.shape(), [2, 2]);
        assert_eq!(f_q.shape(), [2, 2]);
        let peaks = compute_partition_peaks(30);
        assert!(f_q.iter().all(|&x| peaks.iter().any(|&p| (x - p).abs() < 1e-10))); // Discretized
        assert!(!j_q.is_nan()); // Ensure valid J(q)
    }
}
