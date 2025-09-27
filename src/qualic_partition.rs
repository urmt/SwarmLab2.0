use ndarray::{Array1, Array2};

/// Computes Hardy-Ramanujan partition numbers p(n) for n=1 to max_n.
/// Returns normalized Q_n peaks in [0,1].
pub fn compute_partition_peaks(max_n: usize) -> Array1<f64> {
    // Precomputed p(n) from OEIS A000041 (n=1 to 30)
    let partitions = vec![
        1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490,
        627, 792, 1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604,
    ];
    let max_p = *partitions.iter().max().unwrap() as f64;
    Array1::from_vec(
        partitions
            .into_iter()
            .take(max_n.min(30))
            .map(|p| p as f64 / max_p) // Normalize to [0,1]
            .collect(),
    )
}

/// Discretizes a qualic tensor to nearest Q_n peak.
pub fn discretize_to_peaks(tensor: &Array2<f64>, peaks: &Array1<f64>) -> Array2<f64> {
    tensor.mapv(|x| {
        let idx = peaks
            .iter()
            .enumerate()
            .min_by(|&(_, a), &(_, b)| {
                (x - a).abs().partial_cmp(&(x - b).abs()).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap();
        peaks[idx]
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_partition_peaks() {
        let peaks = compute_partition_peaks(5);
        let expected = array![1.0 / 5604.0, 2.0 / 5604.0, 3.0 / 5604.0, 5.0 / 5604.0, 7.0 / 5604.0];
        assert!((peaks - expected).mapv(|x| x.abs()).sum() < 1e-10);
    }

    #[test]
    fn test_discretize() {
        let peaks = compute_partition_peaks(3);
        let tensor = array![[0.1, 0.4], [0.7, 0.2]];
        let discretized = discretize_to_peaks(&tensor, &peaks);
        // Expected: snap to nearest of [1/5604, 2/5604, 3/5604]
        assert!(discretized.iter().all(|&x| peaks.iter().any(|&p| (x - p).abs() < 1e-10)));
    }
}
