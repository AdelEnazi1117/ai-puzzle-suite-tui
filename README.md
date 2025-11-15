# AI Puzzle Suite (TUI)

A terminal-based interactive puzzle suite demonstrating the **A\* (A-Star) search algorithm** through four classic AI problems. Built with Rust and featuring a beautiful terminal user interface using `ratatui`.

![Version](https://img.shields.io/badge/version-1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)

## 🎮 Features

- **Four Interactive Puzzles**:

  - **8-Puzzle Solver** - Sliding tile puzzle with Manhattan distance heuristic
  - **XOR Tic-Tac-Toe** - Strategic game variant with A\* hints
  - **Missionaries & Cannibals** - Classic river crossing problem
  - **8 Queens Problem** - Constraint satisfaction demonstration

- **A\* Algorithm Visualization**:

  - Real-time search statistics (expanded nodes, visited states)
  - Step-by-step solution visualization
  - Heuristic function explanations
  - Educational content about the algorithm

- **User-Friendly Interface**:
  - Intuitive keyboard controls
  - Editable puzzle states
  - Board shuffling and randomization
  - Custom goal state (in 8-Puzzle)

## 📋 Table of Contents

- [Quick Start](#-quick-start)
- [Installation](#-installation)
- [Usage](#-usage)
- [Puzzle Details](#-puzzle-details)
- [Building from Source](#-building-from-source)
- [Distribution Packages](#-distribution-packages)
- [Troubleshooting](#-troubleshooting)
- [Technical Details](#-technical-details)
- [Credits](#-credits)
- [License](#-license)

## 📦 Installation

### Option 1: Pre-built Packages (Recommended)

Download the latest release from the [Releases](https://github.com/AdelEnazi1117/ai-puzzle-suite-tui/releases)

- **macOS**: `ai-puzzle-suite-tui-mac.zip`
- **Windows**: `ai-puzzle-suite-tui-windows.zip`

Extract and follow the instructions in the included `README.txt` file.

### Option 2: Build from Source

#### Prerequisites

- **Rust** (1.70 or later)
  - Install from [rustup.rs](https://rustup.rs/)
  - Or via Homebrew: `brew install rustup-init && rustup-init -y`

#### Build Steps

1. **Clone the repository**:

   ```bash
   git clone https://github.com/AdelEnazi1117/ai-puzzle-suite-tui.git
   cd ai-puzzle-suite-tui
   ```

2. **Build the project**:

   ```bash
   cargo build --release
   ```

3. **Run the application**:

   ```bash
   cargo run --release
   ```

   Or run the binary directly:

   ```bash
   ./target/release/ai-puzzle-suite-tui
   ```

## 🎯 Usage

### Main Menu Controls

- `↑` `↓` - Navigate puzzle list
- `Enter` - Select puzzle
- `Q` - Quit application

### General Puzzle Controls

- `B` - Back to main menu
- `Q` - Quit application
- Controls vary by puzzle (see below)

### 8-Puzzle Controls

- `Tab` - Switch between current board and goal board
- `↑` `↓` `←` `→` - Move cursor
- `1-8` - Place number in selected cell
- `H` - Shuffle current board
- `G` - Shuffle goal board (when editing goal)
- `S` - Solve with A\* algorithm
- `Space` - Step through solution
- `R` - Reset to initial state
- `N` - New random board

### XOR Tic-Tac-Toe Controls

- `Tab` - Toggle setup mode
- `↑` `↓` `←` `→` - Move cursor
- `X` / `O` - Place X or O manually
- `1-9` - Quick place (number pad layout)
- `Space` / `Enter` - Place mark
- `H` - Shuffle board
- `S` - Auto-move (A\* hint)
- `R` - Reset game

### Missionaries & Cannibals Controls

- `↑` `↓` - Navigate valid moves list
- `1-5` - Apply move by number
- `S` - Solve with A\* algorithm
- `Space` - Step through solution
- `H` - Shuffle initial state
- `R` - Reset to initial state

### 8 Queens Controls

- `↑` `↓` `←` `→` - Move cursor
- `Space` - Toggle queen placement
- `S` - Solve with A\* algorithm
- `H` - Shuffle (generates solvable state with 1-4 queens)
- `R` - Reset board

## 🧩 Puzzle Details

### 1. 8-Puzzle Solver

A classic sliding tile puzzle where you arrange numbered tiles in order. The A\* algorithm uses the **Manhattan distance heuristic** to find the optimal solution.

**Features**:

- Editable goal state
- Board shuffling
- Real-time solution visualization
- Statistics: expanded nodes, visited states

### 2. XOR Tic-Tac-Toe

A strategic variant of Tic-Tac-Toe where the goal is to avoid making three in a row. The A\* algorithm provides hints for optimal play.

**Features**:

- Setup mode for custom board states
- Game mode with A\* hints
- Manual X/O placement

### 3. Missionaries & Cannibals

The classic river crossing puzzle: transport 3 missionaries and 3 cannibals across a river using a boat that holds 2 people, ensuring cannibals never outnumber missionaries.

**Features**:

- A\* finds optimal solution
- Step-by-step boat movement visualization
- Random initial state shuffling

### 4. 8 Queens Problem

Place 8 queens on a chessboard such that no two queens attack each other. Demonstrates constraint satisfaction and backtracking.

**Features**:

- Visual chessboard representation
- Conflict detection
- A\* solves from partial states
- Guaranteed solvable shuffle (1-4 queens)

## 🐛 Troubleshooting

### Windows Defender / SmartScreen Warning

If Windows blocks the application:

1. **Right-click** the `.exe` → **Properties** → Check **"Unblock"** (if available)
2. If you see "Windows protected your PC":
   - Click **"More info"**
   - Click **"Run anyway"**
3. This is normal for unsigned applications and safe to allow

### Terminal Display Issues

**macOS**:

- Ensure Terminal.app supports UTF-8
- Try: `export LANG=en_US.UTF-8`

**Windows**:

- Use Command Prompt (cmd.exe) for best compatibility
- Set code page: `chcp 65001`
- Ensure Windows 10+ for ANSI color support

### Colors Not Displaying

- **macOS**: Terminal.app should support 256 colors by default
- **Windows**: Requires Windows 10+ with ANSI support enabled
- Try running in Command Prompt instead of PowerShell

### Application Won't Start

- Ensure you're using a terminal emulator (not just a text editor)
- Check that the binary has execute permissions (macOS/Linux): `chmod +x ai-puzzle-suite-tui`
- Verify Rust version: `rustc --version` (should be 1.70+)

## 🔬 Technical Details

### Architecture

- **Language**: Rust (2021 edition)
- **UI Framework**: [ratatui](https://github.com/ratatui-org/ratatui) (formerly tui-rs)
- **Terminal Backend**: [crossterm](https://github.com/crossterm-rs/crossterm)
- **Error Handling**: [color-eyre](https://github.com/eyreists/color-eyre)

### Algorithm Implementation

- **A\* Search**: Generic implementation in `src/search/solver.rs`
- **SearchState Trait**: Abstract interface for puzzle states
- **Heuristics**:
  - 8-Puzzle: Manhattan distance
  - XOR Tic-Tac-Toe: Game state evaluation
  - Missionaries & Cannibals: Remaining people count
  - 8 Queens: Conflict count

### Project Structure

```
ai-puzzle-suite-tui/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Application state and puzzle sessions
│   ├── ui/
│   │   └── mod.rs           # TUI rendering and input handling
│   ├── puzzles/
│   │   ├── mod.rs           # Puzzle registry
│   │   ├── eight_puzzle.rs  # 8-Puzzle implementation
│   │   ├── xor_tic_tac_toe.rs
│   │   ├── missionaries_cannibals.rs
│   │   └── eight_queens.rs
│   └── search/
│       ├── mod.rs           # Search module exports
│       ├── state.rs         # SearchState trait
│       └── solver.rs        # A* algorithm implementation
├── Cargo.toml               # Rust project configuration
└── README.md                # This file
```

### Dependencies

- `ratatui` - Terminal UI framework
- `crossterm` - Cross-platform terminal manipulation
- `color-eyre` - Error reporting with colors
- `rand` - Random number generation
- `parking_lot` - Fast synchronization primitives

## 👤 Credits

**Developer**: Adel Enazi

**Acknowledgments**:
Special thanks to **Professor Abdulrahman Fakki** for teaching the Artificial Intelligence course that inspired this project.

This application demonstrates the A\* (A-Star) search algorithm through interactive puzzle solving, showcasing heuristic-based pathfinding and constraint satisfaction techniques.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Version 1.0** | November 2025 | Built with Rust
