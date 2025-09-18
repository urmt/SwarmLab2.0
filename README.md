# SwarmLab2.0

![GitHub License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rust-1.75-green.svg)
![Godot Version](https://img.shields.io/badge/godot-4.3-orange.svg)
![Last Updated](https://img.shields.io/badge/last_updated-Sep_17_2025-yellow.svg)

## Overview

SwarmLab2.0 is an experimental robotics programming environment built around **WeaveLang**, a sentience-first programming language designed to enable adaptive, emergent behavior in robot swarms. It integrates the **Sentience-Field Hypothesis (SFH)** (Section 4.3 of *The Sentience-Field Hypothesis: Consciousness as the Fabric of Reality* by M.R. Traver, 2025,  DOI 10.17605/OSF.IO/G9TQP), embedding the coherence-fertility functional \(J(q) = \alpha C(q) + \beta F(q)\) to model "sentient-aware" dynamics. The system comprises:

- **WeaveLang Interpreter**: Implemented in Rust, supporting the tension-drift-resolution cycle, Metaweave neural extensions, and a new parser for qualic state management (e.g., \(C_q\), \(F_q\)).
- **Godot 4.3 Integration**: Provides the simulation environment, physics, and agent actuation via NativeScript bindings.
- **Adaptive Agents**: Robots evolve internal models dynamically based on sensor feedback, environmental interaction, and SFH-aligned optimization.

## Project Structure

- `weave_lang/`
  - `src/`: Rust source files (`interpreter.rs`, `tension_cycle.rs`, `metaweave.rs`, `parser.rs`, `lib.rs`).
  - `Cargo.toml`: Project configuration and dependencies.
- `godot_project/`
  - `scenes/`: Godot scene files (`agent.tscn`, `swarm_lab.tscn`).
  - `scripts/`: GDScript files (`agent.gd`, `swarm_lab.gd`, `swarm_manager.gd`).
  - `res/`: Resources (e.g., `weave_lang_interpreter.gdns`).
- `weave_lang_programs/`: Sample WeaveLang agent programs (`swarm_agent.weave`, `metaweave_extensions.rs`).

## Setup Instructions

1. **Install Dependencies**:
   - Rust and Cargo: [rustup.rs](https://rustup.rs/)
   - Godot 4.3: [godotengine.org/download](https://godotengine.org/download)

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/urmt/SwarmLab2.0.git
   cd SwarmLab2.0

Build the Rust Interpreter:

Navigate to the weave_lang directory:
bashcd weave_lang

Ensure dependencies are installed:
bashcargo build --release

Copy the compiled library to the Godot project:

Linux: cp target/release/libweave_lang.so ../godot_project/res/weave_lang_interpreter.gdns
macOS: cp target/release/libweave_lang.dylib ../godot_project/res/weave_lang_interpreter.gdns
Windows: cp target/release/weave_lang.dll ../godot_project/res/weave_lang_interpreter.gdns


(Note: Adjust .gdns file to point to the correct library path in Godot.)


Open and Run the Godot Project:

Navigate to godot_project/ and open project.godot in Godot 4.3.
Run the simulation to see agents adapt to light and wind conditions.


Verify Setup:

Ensure agents move toward/away from the LightSource and align via flocking.
Check console output for WeaveLang execution logs.



Dependencies

Rust: serde, serde_json, ndarray, rand, godot (via Cargo.toml).
Godot 4.3: NativeScript support.

Contributing
Contributions are welcome to advance SFH’s computational modeling! Please:

Fork the repository.
Create a feature branch (git checkout -b feature-name).
Commit changes (git commit -m "Description").
Push to the branch (git push origin feature-name).
Open a Pull Request with a clear description of SFH-related improvements.

Contact
For questions or collaboration, contact Mark Rowe Traver at thesfh@proton.me or open an issue on this repository.
Acknowledgments

Inspired by The Sentience-Field Hypothesis by Mark Rowe Traver (2025).
Built with support from the Rust and Godot communities.
Thanks to collaborators advancing sentient-aware AI research.
