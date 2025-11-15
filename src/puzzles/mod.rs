pub mod eight_puzzle;
pub mod eight_queens;
pub mod missionaries_cannibals;
pub mod xor_tic_tac_toe;

pub use eight_puzzle::{EightPuzzleState, SlideMove};
pub use eight_queens::{EightQueensState, PlaceQueen};
pub use missionaries_cannibals::{BoatMove, MissionariesCannibalsState};
pub use xor_tic_tac_toe::{Player, XorTicTacToeState, WINNING_LINES};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PuzzleId {
    EightPuzzle,
    XorTicTacToe,
    MissionariesCannibals,
    EightQueens,
    About,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PuzzleDescriptor {
    pub id: PuzzleId,
    pub name: &'static str,
    pub summary: &'static str,
}

#[derive(Debug, Clone)]
pub struct PuzzleRegistry {
    pub descriptors: Vec<PuzzleDescriptor>,
}

impl PuzzleRegistry {
    pub fn descriptor(&self, id: PuzzleId) -> Option<&PuzzleDescriptor> {
        self.descriptors.iter().find(|desc| desc.id == id)
    }
}

impl Default for PuzzleDescriptor {
    fn default() -> Self {
        Self {
            id: PuzzleId::EightPuzzle,
            name: "8-Puzzle Solver",
            summary: "Classic sliding puzzle solved with A* and Manhattan heuristic.",
        }
    }
}

impl Default for PuzzleId {
    fn default() -> Self {
        PuzzleId::EightPuzzle
    }
}

impl PuzzleRegistry {
    pub fn initialize() -> Self {
        let descriptors = vec![
            PuzzleDescriptor {
                id: PuzzleId::EightPuzzle,
                name: "8-Puzzle Solver",
                summary: "Slide tiles into place, observe heuristic-driven search stats.",
            },
            PuzzleDescriptor {
                id: PuzzleId::XorTicTacToe,
                name: "XOR Tic-Tac-Toe",
                summary: "Play optimally with A* hints in an unusual variant of tic-tac-toe.",
            },
            PuzzleDescriptor {
                id: PuzzleId::MissionariesCannibals,
                name: "Missionaries & Cannibals",
                summary: "Get 3 missionaries and 3 cannibals across the river safely using A* search.",
            },
            PuzzleDescriptor {
                id: PuzzleId::EightQueens,
                name: "8 Queens Problem",
                summary: "Place 8 queens on a chessboard so none attack each other. Watch A* solve it!",
            },
            PuzzleDescriptor {
                id: PuzzleId::About,
                name: "About This Program",
                summary: "Learn about this AI Puzzle Suite, the A* algorithm, and acknowledgments.",
            },
        ];

        Self { descriptors }
    }
}

impl Default for PuzzleRegistry {
    fn default() -> Self {
        Self::initialize()
    }
}
