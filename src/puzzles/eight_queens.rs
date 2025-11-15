use crate::search::SearchState;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EightQueensState {
    // Each element represents the column position of the queen in that row
    // queens[row] = column (0-7)
    pub queens: [Option<u8>; 8],
}

impl Default for EightQueensState {
    fn default() -> Self {
        Self {
            queens: [None; 8],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlaceQueen {
    pub row: u8,
    pub col: u8,
}

impl EightQueensState {
    pub fn is_valid_placement(&self, row: u8, col: u8) -> bool {
        // Check if column is already occupied
        for r in 0..8 {
            if let Some(c) = self.queens[r as usize] {
                if c == col {
                    return false;
                }
            }
        }

        // Check diagonals
        for r in 0..8 {
            if let Some(c) = self.queens[r as usize] {
                let row_diff = (r as i8) - (row as i8);
                let col_diff = (c as i8) - (col as i8);
                if row_diff.abs() == col_diff.abs() {
                    return false;
                }
            }
        }

        true
    }

    pub fn count_conflicts(&self) -> u32 {
        let mut conflicts = 0;
        
        // Count column conflicts
        for col in 0..8 {
            let mut count = 0;
            for row in 0..8 {
                if let Some(c) = self.queens[row as usize] {
                    if c == col {
                        count += 1;
                    }
                }
            }
            if count > 1 {
                conflicts += count - 1;
            }
        }

        // Count diagonal conflicts
        for row1 in 0..8 {
            if let Some(col1) = self.queens[row1 as usize] {
                for row2 in (row1 + 1)..8 {
                    if let Some(col2) = self.queens[row2 as usize] {
                        let row_diff = (row2 as i8) - (row1 as i8);
                        let col_diff = (col2 as i8) - (col1 as i8);
                        if row_diff.abs() == col_diff.abs() {
                            conflicts += 1;
                        }
                    }
                }
            }
        }

        conflicts
    }

    pub fn apply_placement(&self, placement: PlaceQueen) -> Option<Self> {
        if placement.row >= 8 || placement.col >= 8 {
            return None;
        }

        if !self.is_valid_placement(placement.row, placement.col) {
            return None;
        }

        let mut new_state = *self;
        new_state.queens[placement.row as usize] = Some(placement.col);
        Some(new_state)
    }

    pub fn remove_queen(&self, row: u8) -> Self {
        let mut new_state = *self;
        if row < 8 {
            new_state.queens[row as usize] = None;
        }
        new_state
    }
}

impl Display for EightQueensState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(q_col) = self.queens[row as usize] {
                    if q_col == col {
                        write!(f, "Q ")?;
                    } else {
                        write!(f, ". ")?;
                    }
                } else {
                    write!(f, ". ")?;
                }
            }
            if row < 7 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl SearchState for EightQueensState {
    type Move = PlaceQueen;

    fn is_goal(&self) -> bool {
        // Goal: all 8 queens placed with no conflicts
        self.queens.iter().all(|q| q.is_some()) && self.count_conflicts() == 0
    }

    fn heuristic(&self) -> u32 {
        // Heuristic: number of conflicts + number of missing queens
        // Plus a penalty for rows with very few valid placements
        let conflicts = self.count_conflicts();
        let missing = self.queens.iter().filter(|q| q.is_none()).count() as u32;
        
        // Count how many empty rows have very few valid placements (penalty)
        let mut penalty = 0u32;
        for row in 0..8 {
            if self.queens[row].is_none() {
                let mut valid_placements = 0u32;
                for col in 0..8 {
                    if self.is_valid_placement(row as u8, col as u8) {
                        valid_placements += 1;
                    }
                }
                // If a row has 0 or 1 valid placements, add penalty
                if valid_placements == 0 {
                    penalty += 10; // Dead end - very bad
                } else if valid_placements == 1 {
                    penalty += 2; // Very constrained
                }
            }
        }
        
        conflicts + missing + penalty
    }

    fn successors(&self) -> Vec<(Self::Move, Self)> {
        let mut successors = Vec::new();
        
        // Find the first empty row
        let empty_row = self.queens.iter().position(|q| q.is_none());
        
        if let Some(row) = empty_row {
            // Try placing a queen in each column of this row
            for col in 0..8 {
                let placement = PlaceQueen {
                    row: row as u8,
                    col,
                };
                if let Some(new_state) = self.apply_placement(placement) {
                    successors.push((placement, new_state));
                }
            }
        }

        successors
    }
}

