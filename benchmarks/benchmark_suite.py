import numpy as np
import pandas as pd
import time
from scipy.stats import ttest_ind
import matplotlib.pyplot as plt

def run_swarm_simulation(agent_count=100, steps=50, discretize=False):
    # Simulated Rust call with Boids-like flocking
    np.random.seed(42)
    positions = np.random.rand(agent_count, 2)
    velocities = np.random.normal(0, 0.1, (agent_count, 2))
    j_values = []
    start_time = time.time()
    for step in range(steps):
        # Tension: Coherence as alignment
        c_q = -np.linalg.norm(velocities - np.mean(velocities, axis=0), axis=1).mean()  # Higher alignment = higher C_q (negated norm)
        # Drift: Add noise to positions
        f_q = positions + np.random.normal(0, 0.2, positions.shape)
        if discretize:
            # Simulate qualic_partition.rs
            partitions = [1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176,
                          231, 297, 385, 490, 627, 792, 1002, 1255, 1575, 1958, 2436,
                          3010, 3718, 4565, 5604]
            peaks = np.array(partitions) / max(partitions)
            f_q = np.array([[peaks[np.argmin(np.abs(peaks - x))] for x in row] for row in f_q])
        # Resolution: J(q) = 0.6 * C_q + 0.4 * F_q (diversity as std)
        f_score = np.std(f_q)
        j_q = 0.6 * c_q + 0.4 * f_score
        j_values.append(j_q)
        # Update velocities (Boids rule simplification)
        velocities += 0.01 * (np.mean(positions, axis=0) - positions)
    latency = (time.time() - start_time) / steps
    iterations = steps  # Full run; in real, stop when ΔJ < 0.01
    return j_values, latency, iterations

def benchmark(num_trials=20, steps=50):
    results = []
    for discretize in [False, True]:
        j_values_list = []
        latencies = []
        iters = []
        for _ in range(num_trials):
            j_values, latency, iterations = run_swarm_simulation(discretize=discretize, steps=steps)
            j_values_list.append(j_values)
            latencies.append(latency)
            iters.append(iterations)
        mean_j = np.mean([np.mean(j) for j in j_values_list])
        std_j = np.std([np.mean(j) for j in j_values_list])
        mean_latency = np.mean(latencies)
        mean_iters = np.mean(iters)
        results.append({
            'mode': 'discretized' if discretize else 'continuous',
            'mean_j': mean_j,
            'std_j': std_j,
            'mean_latency': mean_latency,
            'mean_iters': mean_iters
        })
    
    df = pd.DataFrame(results)
    df.to_csv('benchmarks/benchmark_results.csv', index=False)
    
    # Statistical test
    cont_j = [np.mean(j) for j in j_values_list if not discretize]
    disc_j = [np.mean(j) for j in j_values_list if discretize]
    t_stat, p_val = ttest_ind(cont_j, disc_j)
    print(f"T-test: t={t_stat:.4f}, p={p_val:.4f}")
    
    # Plot convergence
    plt.plot(np.mean(j_values_list[0], axis=0), label='Continuous')
    plt.plot(np.mean(j_values_list[1], axis=0), label='Discretized')
    plt.xlabel('Iteration')
    plt.ylabel('J(q)')
    plt.legend()
    plt.savefig('benchmarks/convergence_plot.png')
    plt.close()
    return df

if __name__ == "__main__":
    benchmark()
