#[cfg(test)]
mod tests {
    use crate::qualic_partition::{compute_partition_peaks, discretize_to_peaks};
    use ndarray::{array, Array2};

    #[test]
    fn test_partition_peaks_accuracy() {
        let peaks = compute_partition_peaks(5);
        let expected = array![1.0 / 5604.0, 2.0 / 5604.0, 3.0 / 5604.0, 5.0 / 5604.0, 7.0 / 5604.0];
        assert!((peaks - &expected).mapv(|x| x.abs()).sum() < 1e-10);
    }

    #[test]
    fn test_discretize_tensor() {
        let peaks = compute_partition_peaks(3);
        let tensor = array![[0.1, 0.4], [0.7, 0.2]];
        let discretized = discretize_to_peaks(&tensor, &peaks);
        assert!(discretized.iter().all(|&x| peaks.iter().any(|&p| (x - p).abs() < 1e-10)));
    }
}
