import numpy as np
import pandas as pd
import time
from scipy.stats import ttest_ind
import matplotlib.pyplot as plt

def run_swarm_simulation(agent_count=100, steps=50, discretize=False):
    # Simulated Rust call (replace with actual Rust FFI in production)
    np.random.seed(42)
    sensors = np.random.rand(agent_count, 8)  # 8D sensor inputs
    weights = np.ones((agent_count, 8)) * 0.7
    c_q = sensors @ weights.T
    f_q = c_q + np.random.normal(0.5, 0.2, c_q.shape)
    
    if discretize:
        # Simulate qualic_partition.rs
        partitions = [1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176,
                      231, 297, 385, 490, 627, 792, 1002, 1255, 1575, 1958, 2436,
                      3010, 3718, 4565, 5604]
        peaks = np.array(partitions) / max(partitions)
        f_q = np.array([[peaks[np.argmin(np.abs(peaks - x))] for x in row] for row in f_q])
    
    c_score = np.mean(c_q)
    f_score = np.std(f_q)
    j_q = 0.6 * c_score + 0.4 * f_score
    return j_q, c_q, f_q

def benchmark(num_trials=20, steps=50):
    results = []
    for discretize in [False, True]:
        j_values = []
        start_time = time.time()
        for _ in range(num_trials):
            j_q, _, _ = run_swarm_simulation(discretize=discretize, steps=steps)
            j_values.append(j_q)
        latency = (time.time() - start_time) / num_trials
        results.append({
            'mode': 'discretized' if discretize else 'continuous',
            'mean_j': np.mean(j_values),
            'std_j': np.std(j_values),
            'latency': latency,
            'iterations': steps  # Placeholder; actual convergence requires Rust
        })
    
    df = pd.DataFrame(results)
    df.to_csv('benchmarks/benchmark_results.csv', index=False)
    
    # Statistical test
    cont_j = [run_swarm_simulation(discretize=False)[0] for _ in range(num_trials)]
    disc_j = [run_swarm_simulation(discretize=True)[0] for _ in range(num_trials)]
    t_stat, p_val = ttest_ind(cont_j, disc_j)
    print(f"T-test: t={t_stat:.4f}, p={p_val:.4f}")
    
    # Plot convergence
    plt.plot(cont_j, label='Continuous')
    plt.plot(disc_j, label='Discretized')
    plt.xlabel('Trial')
    plt.ylabel('J(q)')
    plt.legend()
    plt.savefig('benchmarks/convergence_plot.png')
    plt.close()
    return df

if __name__ == "__main__":
    benchmark()
