use crate::search::SearchState;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MissionariesCannibalsState {
    // Left side: missionaries, cannibals
    pub left_m: u8,
    pub left_c: u8,
    // Boat position: true = left, false = right
    pub boat_left: bool,
}

impl Default for MissionariesCannibalsState {
    fn default() -> Self {
        Self {
            left_m: 3,
            left_c: 3,
            boat_left: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoatMove {
    pub missionaries: u8,
    pub cannibals: u8,
}

impl MissionariesCannibalsState {
    pub fn is_valid(&self) -> bool {
        // Check left side
        if self.left_m > 0 && self.left_c > self.left_m {
            return false;
        }
        // Check right side
        let right_m = 3 - self.left_m;
        let right_c = 3 - self.left_c;
        if right_m > 0 && right_c > right_m {
            return false;
        }
        // Check boat capacity
        true
    }

    pub fn apply_move(&self, mv: BoatMove) -> Option<Self> {
        if mv.missionaries + mv.cannibals == 0 || mv.missionaries + mv.cannibals > 2 {
            return None;
        }

        let mut new_state = *self;

        if self.boat_left {
            // Moving from left to right
            if mv.missionaries > self.left_m || mv.cannibals > self.left_c {
                return None;
            }
            new_state.left_m -= mv.missionaries;
            new_state.left_c -= mv.cannibals;
            new_state.boat_left = false;
        } else {
            // Moving from right to left
            let right_m = 3 - self.left_m;
            let right_c = 3 - self.left_c;
            if mv.missionaries > right_m || mv.cannibals > right_c {
                return None;
            }
            new_state.left_m += mv.missionaries;
            new_state.left_c += mv.cannibals;
            new_state.boat_left = true;
        }

        if new_state.is_valid() {
            Some(new_state)
        } else {
            None
        }
    }

    pub fn heuristic(&self) -> u32 {
        // Heuristic: number of people on left side (all need to cross)
        (self.left_m + self.left_c) as u32
    }
}

impl Display for MissionariesCannibalsState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let right_m = 3 - self.left_m;
        let right_c = 3 - self.left_c;
        
        writeln!(f, "Left:  M={} C={}", self.left_m, self.left_c)?;
        writeln!(f, "Right: M={} C={}", right_m, right_c)?;
        writeln!(f, "Boat:  {}", if self.boat_left { "Left" } else { "Right" })?;
        Ok(())
    }
}

impl SearchState for MissionariesCannibalsState {
    type Move = BoatMove;

    fn is_goal(&self) -> bool {
        self.left_m == 0 && self.left_c == 0 && !self.boat_left
    }

    fn heuristic(&self) -> u32 {
        self.heuristic()
    }

    fn successors(&self) -> Vec<(Self::Move, Self)> {
        let mut moves = Vec::new();
        
        // Generate all possible boat moves (1-2 people, at least 1 person)
        let possible_moves = vec![
            BoatMove { missionaries: 1, cannibals: 0 },
            BoatMove { missionaries: 2, cannibals: 0 },
            BoatMove { missionaries: 0, cannibals: 1 },
            BoatMove { missionaries: 0, cannibals: 2 },
            BoatMove { missionaries: 1, cannibals: 1 },
        ];

        for mv in possible_moves {
            if let Some(new_state) = self.apply_move(mv) {
                moves.push((mv, new_state));
            }
        }

        moves
    }
}

