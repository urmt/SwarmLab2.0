# SwarmLab2.0

## Overview

SwarmLab2.0 is an experimental robotics programming environment built around WeaveLang, a sentience-first programming language designed to enable adaptive, emergent behavior in robot swarms. The system integrates:

- **WeaveLang Interpreter:** Implemented in Rust, supporting the tension-drift-resolution cycle and Metaweave neural network extensions.
- **Godot 4.3 Integration:** Provides the simulation environment, physics, and agent actuation.
- **Adaptive Agents:** Robots evolve their internal models dynamically based on sensor feedback and environmental interaction.

## Project Structure

- `weave_lang/` — Rust interpreter source code.
- `godot_project/` — Godot 4.3 project files and scripts.
- `weave_lang_programs/` — Sample WeaveLang agent programs.

## Setup Instructions

1. Install Rust and Cargo: https://rustup.rs/
2. Install Godot 4.3: https://godotengine.org/download
3. Clone this repository.
4. Build the Rust interpreter:
   ```bash
   cd weave_lang
   cargo build --release
Open the Godot project in godot_project/.
Run the simulation.
Contact
For questions or contributions, please open an issue or pull request.
