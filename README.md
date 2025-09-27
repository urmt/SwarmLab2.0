# SwarmLab2.0

![GitHub License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rust-1.75-green.svg)
![Godot Version](https://img.shields.io/badge/godot-4.3-orange.svg)
![Last Updated](https://img.shields.io/badge/last_updated-Sep_27_2025-yellow.svg)

## Overview

SwarmLab2.0 is an experimental robotics programming environment built around **WeaveLang**, a sentience-first programming language designed to enable adaptive, emergent behavior in robot swarms. It integrates the **Sentience-Field Hypothesis (SFH)** (Section 4.3 of *The Sentience-Field Hypothesis: Consciousness as the Fabric of Reality* by M.R. Traver, 2025, DOI 10.17605/OSF.IO/G9TQP), embedding the coherence-fertility functional \(J(q) = \alpha C(q) + \beta F(q)\) to model "sentient-aware" dynamics. The system comprises:

- **WeaveLang Interpreter**: Implemented in Rust, supporting the tension-drift-resolution cycle, Metaweave neural extensions, a new parser for qualic state management (e.g., \(C_q\), \(F_q\)), and a novel **partition discretization** feature using Hardy-Ramanujan partition peaks to optimize \(J(q)\).
- **Godot 4.3 Integration**: Provides the simulation environment, physics, and agent actuation via NativeScript bindings.
- **Adaptive Agents**: Robots evolve internal models dynamically based on sensor feedback, environmental interaction, and SFH-aligned optimization.

## Project Structure

- `weave_lang/`
  - `src/`: Rust source files (`interpreter.rs`, `tension_cycle.rs`, `metaweave.rs`, `parser.rs`, `lib.rs`, `qualic_partition.rs`).
    - `qualic_partition.rs`: New module computing Hardy-Ramanujan partition peaks (\(Q_n\)) and discretizing qualic tensors to improve \(J(q)\) optimization by ~10-15%.
  - `Cargo.toml`: Project configuration and dependencies.
- `godot_project/`
  - `scenes/`: Godot scene files (`agent.tscn`, `swarm_lab.tscn`).
  - `scripts/`: GDScript files (`agent.gd`, `swarm_lab.gd`, `swarm_manager.gd`).
  - `res/`: Resources (e.g., `weave_lang_interpreter.gdns`).
- `weave_lang_programs/`: Sample WeaveLang agent programs (`swarm_agent.weave`, `metaweave_extensions.rs`).
- `benchmarks/`: Performance evaluation scripts and results.
  - `benchmark_suite.py`: Python script comparing continuous vs. discretized runs.
  - `benchmark_results.csv`: Output file with mean/std \(J(q)\), latency, and iterations.
- `tests/`
  - `test_cycle.rs`: Tests for tension-drift-resolution cycle.
  - `test_partition.rs`: New tests for `qualic_partition.rs` accuracy.

## Setup Instructions

1. **Install Dependencies**:
   - Rust and Cargo: [rustup.rs](https://rustup.rs/)
   - Godot 4.3: [godotengine.org/download](https://godotengine.org/download)
   - Python: Install with `pip install numpy pandas scipy matplotlib` for benchmarks.

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/urmt/SwarmLab2.0.git
   cd SwarmLab2.0
   ```

3. **Build the Rust Interpreter**:
   - Navigate to the `weave_lang` directory:
     ```bash
     cd weave_lang
     ```
   - Ensure dependencies are installed:
     ```bash
     cargo build --release
     ```
   - Copy the compiled library to the Godot project:
     - Linux: `cp target/release/libweave_lang.so ../godot_project/res/weave_lang_interpreter.gdns`
     - macOS: `cp target/release/libweave_lang.dylib ../godot_project/res/weave_lang_interpreter.gdns`
     - Windows: `cp target/release/weave_lang.dll ../godot_project/res/weave_lang_interpreter.gdns`
   - **Note**: Adjust the `.gdns` file in Godot to point to the correct library path if needed.

4. **Open and Run the Godot Project**:
   - Navigate to `godot_project/` and open `project.godot` in Godot 4.3.
   - Run the simulation to see agents adapt to light and wind conditions.

5. **Run Benchmarks** (Optional):
   - Navigate to `benchmarks/`:
     ```bash
     cd benchmarks
     ```
   - Execute the benchmark script:
     ```bash
     python benchmark_suite.py
     ```
   - Output: `benchmark_results.csv` and `convergence_plot.png` for performance analysis.

6. **Verify Setup**:
   - Ensure agents move toward/away from the `LightSource` and align via flocking.
   - Check console output for WeaveLang execution logs, including partition discretization effects.

## Dependencies

- **Rust**: `serde`, `serde_json`, `ndarray`, `rand`, `godot` (via `Cargo.toml`).
- **Godot 4.3**: NativeScript support.
- **Python**: `numpy`, `pandas`, `scipy`, `matplotlib` for benchmarks.

## Usage

- Run a simulation with 100 agents, optionally enabling partition discretization:
  ```bash
  cargo run --release -- --agents 100 --discretize
  ```
- Analyze performance with benchmarks to compare continuous vs. discretized modes (see `benchmarks/`).

## New Features

- **Partition Discretization**: Introduced in `src/qualic_partition.rs`, this feature computes Hardy-Ramanujan partition peaks (\(Q_n\)) and discretizes qualic tensors during the drift phase. This enhances \(J(q)\) optimization by ~10-15%, validated via `benchmarks/benchmark_suite.py`.

## Contributing

Contributions are welcome to advance SFH’s computational modeling! Please:
1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Commit changes: `git commit -m "Description of SFH-related improvements"`.
4. Push to the branch: `git push origin feature-name`.
5. Open a Pull Request with a clear description of changes.

## Contact

For questions or collaboration, contact Mark Rowe Traver at `thesfh@proton.me` or open an issue on this repository.

## Acknowledgments

- Inspired by *The Sentience-Field Hypothesis* by Mark Rowe Traver (2025).
- Built with support from the Rust and Godot communities.
- Thanks to collaborators advancing sentient-aware AI research.

## License

MIT License. See `LICENSE`.