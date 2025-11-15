use crate::search::SearchState;
use serde::{Deserialize, Serialize};

pub const WINNING_LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn opponent(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XorTicTacToeState {
    pub cells: [Option<Player>; 9],
    pub to_move: Player,
}

impl Default for XorTicTacToeState {
    fn default() -> Self {
        Self {
            cells: [None; 9],
            to_move: Player::X,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct PlaceMove {
    pub index: usize,
}

impl XorTicTacToeState {
    pub fn winner(&self) -> Option<Player> {
        for line in WINNING_LINES {
            if let (Some(a), Some(b), Some(c)) = (
                self.cells[line[0]],
                self.cells[line[1]],
                self.cells[line[2]],
            ) {
                if a == b && b == c {
                    return Some(a);
                }
            }
        }
        None
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_some())
    }
}

impl SearchState for XorTicTacToeState {
    type Move = PlaceMove;

    fn is_goal(&self) -> bool {
        matches!(self.winner(), Some(Player::X))
    }

    fn heuristic(&self) -> u32 {
        match self.winner() {
            Some(Player::X) => 0,
            Some(Player::O) => 100,
            None => {
                let center_bonus = match self.cells[4] {
                    Some(Player::X) => 0,
                    Some(Player::O) => 4,
                    None => 2,
                };
                center_bonus
            }
        }
    }

    fn successors(&self) -> Vec<(Self::Move, Self)> {
        if self.winner().is_some() || self.is_full() {
            return Vec::new();
        }

        self.cells
            .iter()
            .enumerate()
            .filter_map(|(idx, cell)| {
                if cell.is_none() {
                    let mut next = *self;
                    next.cells[idx] = Some(self.to_move);
                    next.to_move = self.to_move.opponent();
                    Some((PlaceMove { index: idx }, next))
                } else {
                    None
                }
            })
            .collect()
    }
}
