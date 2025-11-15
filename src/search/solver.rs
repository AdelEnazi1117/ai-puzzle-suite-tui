use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::{Duration, Instant};

use super::SearchState;

#[derive(Debug, Clone)]
pub struct SearchReport<S: SearchState> {
    pub path: Vec<S>,
    pub expanded_nodes: usize,
    pub visited_states: usize,
    pub goal_found: bool,
    pub elapsed: Duration,
}

impl<S: SearchState> Default for SearchReport<S> {
    fn default() -> Self {
        Self {
            path: Vec::new(),
            expanded_nodes: 0,
            visited_states: 0,
            goal_found: false,
            elapsed: Duration::default(),
        }
    }
}

#[derive(Clone)]
struct FrontierEntry<S: SearchState> {
    state: S,
    g_cost: u32,
    h_cost: u32,
}

impl<S: SearchState> FrontierEntry<S> {
    fn f_cost(&self) -> u32 {
        self.g_cost + self.h_cost
    }
}

impl<S: SearchState> Eq for FrontierEntry<S> {}

impl<S: SearchState> PartialEq for FrontierEntry<S> {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost() == other.f_cost() && self.h_cost == other.h_cost
    }
}

impl<S: SearchState> Ord for FrontierEntry<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering because BinaryHeap is a max-heap by default.
        other
            .f_cost()
            .cmp(&self.f_cost())
            .then_with(|| other.h_cost.cmp(&self.h_cost))
    }
}

impl<S: SearchState> PartialOrd for FrontierEntry<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn astar<S: SearchState>(start: S) -> SearchReport<S> {
    const MAX_TIME: Duration = Duration::from_secs(3600); // 1 hour timeout
    
    let start_time = Instant::now();
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<S, (Option<S>, u32)> = HashMap::new();

    open.push(FrontierEntry {
        g_cost: 0,
        h_cost: start.heuristic(),
        state: start.clone(),
    });
    came_from.insert(start.clone(), (None, 0));

    let mut expanded = 0usize;

    while let Some(entry) = open.pop() {
        // Check timeout (1 hour max)
        if start_time.elapsed() >= MAX_TIME {
            return SearchReport {
                path: Vec::new(),
                expanded_nodes: expanded,
                visited_states: came_from.len(),
                goal_found: false,
                elapsed: start_time.elapsed(),
            };
        }
        
        let current_state = entry.state;

        let (_, recorded_cost) = came_from
            .get(&current_state)
            .cloned()
            .unwrap_or((None, u32::MAX));

        if entry.g_cost > recorded_cost {
            continue;
        }

        if current_state.is_goal() {
            return SearchReport {
                path: reconstruct_path(&came_from, current_state),
                expanded_nodes: expanded,
                visited_states: came_from.len(),
                goal_found: true,
                elapsed: start_time.elapsed(),
            };
        }

        expanded += 1;

        for (_, successor) in current_state.successors() {
            let tentative_cost = entry.g_cost.saturating_add(1);
            let needs_update = match came_from.get(&successor) {
                Some((_, known_cost)) => tentative_cost < *known_cost,
                None => true,
            };

            if needs_update {
                came_from.insert(
                    successor.clone(),
                    (Some(current_state.clone()), tentative_cost),
                );
                open.push(FrontierEntry {
                    h_cost: successor.heuristic(),
                    g_cost: tentative_cost,
                    state: successor,
                });
            }
        }
    }

    SearchReport {
        path: Vec::new(),
        expanded_nodes: expanded,
        visited_states: came_from.len(),
        goal_found: false,
        elapsed: start_time.elapsed(),
    }
}

fn reconstruct_path<S: SearchState>(
    came_from: &HashMap<S, (Option<S>, u32)>,
    mut current: S,
) -> Vec<S> {
    let mut path = vec![current.clone()];
    while let Some((Some(parent), _)) = came_from.get(&current) {
        current = parent.clone();
        path.push(current.clone());
    }
    path.reverse();
    path
}
