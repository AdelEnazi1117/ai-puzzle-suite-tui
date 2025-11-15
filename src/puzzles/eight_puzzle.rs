use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt::{Display, Formatter};

use crate::search::SearchState;

const GOAL: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 0];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EightPuzzleState {
    pub tiles: [u8; 9],
}

impl Default for EightPuzzleState {
    fn default() -> Self {
        Self { tiles: GOAL }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SlideMove {
    Up,
    Down,
    Left,
    Right,
}

impl SlideMove {
    pub fn label(&self) -> &'static str {
        match self {
            SlideMove::Up => "Up",
            SlideMove::Down => "Down",
            SlideMove::Left => "Left",
            SlideMove::Right => "Right",
        }
    }
}

impl EightPuzzleState {
    pub fn random_solvable(rng: &mut impl Rng) -> Self {
        let mut tiles = GOAL;
        loop {
            tiles.shuffle(rng);
            if is_solvable(&tiles) {
                return Self { tiles };
            }
        }
    }

    pub fn blank_index(&self) -> usize {
        self.tiles.iter().position(|&t| t == 0).unwrap_or(8)
    }

    pub fn manhattan_distance(&self) -> u32 {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, &tile)| tile != 0)
            .map(|(idx, &tile)| {
                let goal_idx = (tile - 1) as usize;
                let (row, col) = (idx / 3, idx % 3);
                let (goal_row, goal_col) = (goal_idx / 3, goal_idx % 3);
                (row.abs_diff(goal_row) + col.abs_diff(goal_col)) as u32
            })
            .sum()
    }

    pub fn apply_move(&self, mv: SlideMove) -> Option<Self> {
        let blank = self.blank_index();
        let row = blank / 3;
        let col = blank % 3;
        let target = match mv {
            SlideMove::Up if row > 0 => Some(blank - 3),
            SlideMove::Down if row < 2 => Some(blank + 3),
            SlideMove::Left if col > 0 => Some(blank - 1),
            SlideMove::Right if col < 2 => Some(blank + 1),
            _ => None,
        }?;

        let mut tiles = self.tiles;
        tiles.swap(blank, target);
        Some(Self { tiles })
    }
}

impl Display for EightPuzzleState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..3 {
            for col in 0..3 {
                let tile = self.tiles[row * 3 + col];
                if tile == 0 {
                    write!(f, "   ")?;
                } else {
                    write!(f, "{:>2} ", tile)?;
                }
            }
            if row < 2 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl SearchState for EightPuzzleState {
    type Move = SlideMove;

    fn is_goal(&self) -> bool {
        self.tiles == GOAL
    }

    fn heuristic(&self) -> u32 {
        self.manhattan_distance()
    }

    fn successors(&self) -> Vec<(Self::Move, Self)> {
        let mut next_states = Vec::new();
        let blank = self.blank_index();
        let row = blank / 3;
        let col = blank % 3;

        let mut push_state = |mv: SlideMove, target_idx: usize| {
            let mut new_tiles = self.tiles;
            new_tiles.swap(blank, target_idx);
            next_states.push((mv, Self { tiles: new_tiles }));
        };

        if row > 0 {
            push_state(SlideMove::Up, blank - 3);
        }
        if row < 2 {
            push_state(SlideMove::Down, blank + 3);
        }
        if col > 0 {
            push_state(SlideMove::Left, blank - 1);
        }
        if col < 2 {
            push_state(SlideMove::Right, blank + 1);
        }

        next_states
    }
}

fn is_solvable(tiles: &[u8; 9]) -> bool {
    let mut inversions = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            if tiles[i] != 0 && tiles[j] != 0 && tiles[i] > tiles[j] {
                inversions += 1;
            }
        }
    }
    inversions % 2 == 0
}
