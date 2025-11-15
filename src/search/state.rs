use std::hash::Hash;

/// Trait implemented by every puzzle state that can be explored by A*.
pub trait SearchState: Clone + Eq + Hash {
    type Move: Clone;

    fn is_goal(&self) -> bool;
    fn heuristic(&self) -> u32;
    fn successors(&self) -> Vec<(Self::Move, Self)>;
}
