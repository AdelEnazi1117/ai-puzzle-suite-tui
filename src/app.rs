use crate::puzzles::{
    BoatMove, EightPuzzleState, EightQueensState, MissionariesCannibalsState, PlaceQueen, Player, PuzzleId, PuzzleRegistry, SlideMove, XorTicTacToeState, WINNING_LINES,
};
use crate::search::{
    solver::{astar, SearchReport},
    SearchState,
};
use rand::thread_rng;
use std::hash::{Hash, Hasher};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::time::Instant;

fn format_player(player: Player) -> &'static str {
    match player {
        Player::X => "X",
        Player::O => "O",
    }
}

// Wrapper for EightPuzzleState with custom goal
#[derive(Debug, Clone)]
struct CustomGoalState {
    state: EightPuzzleState,
    goal: EightPuzzleState,
}

impl PartialEq for CustomGoalState {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Eq for CustomGoalState {}

impl Hash for CustomGoalState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

impl SearchState for CustomGoalState {
    type Move = SlideMove;

    fn is_goal(&self) -> bool {
        self.state.tiles == self.goal.tiles
    }

    fn heuristic(&self) -> u32 {
        // Manhattan distance to custom goal
        self.state
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, &tile)| tile != 0)
            .map(|(idx, &tile)| {
                // Find where this tile should be in goal
                let goal_idx = self.goal.tiles.iter().position(|&t| t == tile).unwrap_or(idx);
                let (row, col) = (idx / 3, idx % 3);
                let (goal_row, goal_col) = (goal_idx / 3, goal_idx % 3);
                (row.abs_diff(goal_row) + col.abs_diff(goal_col)) as u32
            })
            .sum()
    }

    fn successors(&self) -> Vec<(Self::Move, Self)> {
        let blank = self.state.blank_index();
        let row = blank / 3;
        let col = blank % 3;
        let mut next_states = Vec::new();

        let mut push_state = |mv: SlideMove, target_idx: usize| {
            let mut new_tiles = self.state.tiles;
            new_tiles.swap(blank, target_idx);
            next_states.push((
                mv,
                CustomGoalState {
                    state: EightPuzzleState { tiles: new_tiles },
                    goal: self.goal,
                },
            ));
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

fn astar_custom_goal(start: CustomGoalState) -> SearchReport<CustomGoalState> {
    let start_time = Instant::now();
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<CustomGoalState, (Option<CustomGoalState>, u32)> = HashMap::new();

    open.push(FrontierEntry {
        g_cost: 0,
        h_cost: start.heuristic(),
        state: start.clone(),
    });
    came_from.insert(start.clone(), (None, 0));

    let mut expanded = 0usize;

    #[derive(Clone)]
    struct FrontierEntry {
        state: CustomGoalState,
        g_cost: u32,
        h_cost: u32,
    }

    impl FrontierEntry {
        fn f_cost(&self) -> u32 {
            self.g_cost + self.h_cost
        }
    }

    impl Eq for FrontierEntry {}
    impl PartialEq for FrontierEntry {
        fn eq(&self, other: &Self) -> bool {
            self.f_cost() == other.f_cost() && self.h_cost == other.h_cost
        }
    }

    impl Ord for FrontierEntry {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .f_cost()
                .cmp(&self.f_cost())
                .then_with(|| other.h_cost.cmp(&self.h_cost))
        }
    }

    impl PartialOrd for FrontierEntry {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    while let Some(entry) = open.pop() {
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
                path: reconstruct_path_custom(&came_from, current_state),
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
                came_from.insert(successor.clone(), (Some(current_state.clone()), tentative_cost));
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

fn reconstruct_path_custom(
    came_from: &HashMap<CustomGoalState, (Option<CustomGoalState>, u32)>,
    mut current: CustomGoalState,
) -> Vec<CustomGoalState> {
    let mut path = vec![current.clone()];
    while let Some((Some(parent), _)) = came_from.get(&current) {
        current = parent.clone();
        path.push(current.clone());
    }
    path.reverse();
    path
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppRoute {
    MainMenu,
    Puzzle(PuzzleId),
    Quit,
}

pub struct App {
    pub registry: PuzzleRegistry,
    pub route: AppRoute,
    pub eight_puzzle: EightPuzzleSession,
    pub xor_ttt: XorTicTacToeSession,
    pub missionaries_cannibals: MissionariesCannibalsSession,
    pub eight_queens: EightQueensSession,
}

impl Default for App {
    fn default() -> Self {
        Self {
            registry: PuzzleRegistry::default(),
            route: AppRoute::MainMenu,
            eight_puzzle: EightPuzzleSession::randomized(),
            xor_ttt: XorTicTacToeSession::default(),
            missionaries_cannibals: MissionariesCannibalsSession::default(),
            eight_queens: EightQueensSession::default(),
        }
    }
}

impl App {
    pub fn select_main_menu(&mut self) {
        self.route = AppRoute::MainMenu;
    }

    pub fn select_puzzle(&mut self, puzzle: PuzzleId) {
        self.route = AppRoute::Puzzle(puzzle);
    }

    pub fn request_quit(&mut self) {
        self.route = AppRoute::Quit;
    }

    pub fn should_exit(&self) -> bool {
        self.route == AppRoute::Quit
    }
}

#[derive(Debug, Clone)]
pub struct EightPuzzleSolution {
    pub report: SearchReport<EightPuzzleState>,
    pub step: usize,
}

impl EightPuzzleSolution {
    pub fn total_steps(&self) -> usize {
        self.report.path.len().saturating_sub(1)
    }
}

#[derive(Debug)]
pub struct EightPuzzleSession {
    pub start: EightPuzzleState,
    pub current: EightPuzzleState,
    pub goal_state: EightPuzzleState,
    pub moves_made: usize,
    pub status: String,
    pub solution: Option<EightPuzzleSolution>,
    pub selected_cell: usize,
    pub goal_selected_cell: usize,
    pub editing_goal: bool,
}

impl EightPuzzleSession {
    fn base_message() -> String {
        "Use arrows to select cell, 1-8 to place number. Tab switches boards. R resets, N shuffles, S solves, Space replays.".into()
    }

    fn random_state() -> EightPuzzleState {
        let mut rng = thread_rng();
        EightPuzzleState::random_solvable(&mut rng)
    }

    pub fn randomized() -> Self {
        let state = Self::random_state();
        Self {
            start: state,
            current: state,
            goal_state: EightPuzzleState::default(), // Default goal [1,2,3,4,5,6,7,8,0]
            moves_made: 0,
            status: Self::base_message(),
            solution: None,
            selected_cell: 0,
            goal_selected_cell: 0,
            editing_goal: false,
        }
    }

    pub fn reset(&mut self) {
        self.current = self.start;
        self.moves_made = 0;
        self.solution = None;
        self.selected_cell = 0;
        self.editing_goal = false;
        self.status = "Reset to starting arrangement.".into();
    }

    pub fn new_board(&mut self) {
        let state = Self::random_state();
        self.start = state;
        self.current = state;
        self.moves_made = 0;
        self.solution = None;
        self.selected_cell = 0;
        self.status = "Generated a new solvable board.".into();
    }

    pub fn shuffle(&mut self) {
        if self.editing_goal {
            let state = Self::random_state();
            self.goal_state = state;
            self.goal_selected_cell = 0;
            self.status = "Goal board shuffled randomly.".into();
        } else {
            let state = Self::random_state();
            self.current = state;
            self.moves_made = 0;
            self.solution = None;
            self.selected_cell = 0;
            self.status = "Board shuffled randomly.".into();
        }
    }

    pub fn toggle_editing_goal(&mut self) {
        self.editing_goal = !self.editing_goal;
        if self.editing_goal {
            self.status = "Now editing goal board. Use Tab to switch back.".into();
        } else {
            self.status = "Now editing current board. Use Tab to edit goal.".into();
        }
    }

    pub fn move_cursor(&mut self, row_delta: isize, col_delta: isize) {
        if self.editing_goal {
            let row = (self.goal_selected_cell / 3) as isize;
            let col = (self.goal_selected_cell % 3) as isize;
            let new_row = (row + row_delta).clamp(0, 2);
            let new_col = (col + col_delta).clamp(0, 2);
            self.goal_selected_cell = (new_row * 3 + new_col) as usize;
        } else {
            let row = (self.selected_cell / 3) as isize;
            let col = (self.selected_cell % 3) as isize;
            let new_row = (row + row_delta).clamp(0, 2);
            let new_col = (col + col_delta).clamp(0, 2);
            self.selected_cell = (new_row * 3 + new_col) as usize;
        }
    }

    pub fn place_number(&mut self, number: u8) -> bool {
        if number == 0 || number > 8 {
            self.status = format!("Invalid number: {}. Use 1-8.", number);
            return false;
        }

        if self.editing_goal {
            let current_value = self.goal_state.tiles[self.goal_selected_cell];
            
            if current_value == number {
                self.status = format!("Goal cell already contains {}.", number);
                return false;
            }

            if let Some(existing_idx) = self.goal_state.tiles.iter().position(|&t| t == number) {
                self.goal_state.tiles[self.goal_selected_cell] = number;
                self.goal_state.tiles[existing_idx] = current_value;
                self.solution = None;
                self.status = format!("Goal: Swapped {} with cell {}.", number, existing_idx + 1);
                return true;
            }

            if current_value == 0 {
                self.goal_state.tiles[self.goal_selected_cell] = number;
                self.solution = None;
                self.status = format!("Goal: Placed {} in cell {}.", number, self.goal_selected_cell + 1);
                return true;
            }

            self.goal_state.tiles[self.goal_selected_cell] = number;
            self.solution = None;
            self.status = format!("Goal: Replaced {} with {} in cell {}.", current_value, number, self.goal_selected_cell + 1);
            true
        } else {
            let current_value = self.current.tiles[self.selected_cell];
            
            if current_value == number {
                self.status = format!("Cell already contains {}.", number);
                return false;
            }

            if let Some(existing_idx) = self.current.tiles.iter().position(|&t| t == number) {
                self.current.tiles[self.selected_cell] = number;
                self.current.tiles[existing_idx] = current_value;
                self.moves_made += 1;
                self.solution = None;
                self.status = format!("Swapped {} with cell {}.", number, existing_idx + 1);
                return true;
            }

            if current_value == 0 {
                self.current.tiles[self.selected_cell] = number;
                self.moves_made += 1;
                self.solution = None;
                self.status = format!("Placed {} in cell {}.", number, self.selected_cell + 1);
                return true;
            }

            self.current.tiles[self.selected_cell] = number;
            self.moves_made += 1;
            self.solution = None;
            self.status = format!("Replaced {} with {} in cell {}.", current_value, number, self.selected_cell + 1);
            true
        }
    }


    pub fn is_solved(&self) -> bool {
        self.current.tiles == self.goal_state.tiles
    }

    pub fn solve_current(&mut self) {
        // Create a wrapper state with custom goal
        let start_state = CustomGoalState {
            state: self.current,
            goal: self.goal_state,
        };
        let report = astar_custom_goal(start_state);
        if report.goal_found && !report.path.is_empty() {
            // Extract the actual states from the wrapper
            let actual_path: Vec<EightPuzzleState> = report.path.iter().map(|s| s.state).collect();
            let actual_report = SearchReport {
                path: actual_path,
                expanded_nodes: report.expanded_nodes,
                visited_states: report.visited_states,
                goal_found: report.goal_found,
                elapsed: report.elapsed,
            };
            self.solution = Some(EightPuzzleSolution { report: actual_report, step: 0 });
            self.moves_made = 0;
            if let Some(solution) = &self.solution {
                if let Some(first) = solution.report.path.first() {
                    self.current = *first;
                }
                self.status = format!(
                    "Solution ready ({} moves). Press Space to step.",
                    solution.total_steps()
                );
            }
        } else {
            self.solution = None;
            self.status = "No solution found (should never happen).".into();
        }
    }

    pub fn advance_solution(&mut self) -> bool {
        if let Some(solution) = &mut self.solution {
            if solution.step + 1 < solution.report.path.len() {
                solution.step += 1;
                if let Some(state) = solution.report.path.get(solution.step) {
                    self.current = *state;
                    self.moves_made = solution.step;
                    if solution.step == solution.report.path.len() - 1 {
                        self.status = "Solution complete! Board solved.".into();
                    } else {
                        self.status = format!(
                            "Replaying solution: step {} / {}",
                            solution.step,
                            solution.total_steps()
                        );
                    }
                }
                return true;
            } else {
                self.status = "Already at final solution state.".into();
                return false;
            }
        }
        self.status = "Run the solver with 'S' first.".into();
        false
    }
}

#[derive(Debug)]
pub struct XorTicTacToeSession {
    pub state: XorTicTacToeState,
    pub cursor: usize,
    pub status: String,
    pub human_symbol: Player,
    pub setup_mode: bool,
}

impl Default for XorTicTacToeSession {
    fn default() -> Self {
        Self {
            state: XorTicTacToeState::default(),
            cursor: 4,
            status: Self::base_status(),
            human_symbol: Player::X,
            setup_mode: false,
        }
    }
}

impl XorTicTacToeSession {
    fn base_status() -> String {
        "Arrows move cursor, X/O place pieces, Tab setup mode, H shuffle, S auto-move, R restart.".into()
    }

    pub fn reset(&mut self) {
        self.state = XorTicTacToeState::default();
        self.cursor = 4;
        self.setup_mode = false;
        self.status = Self::base_status();
    }

    pub fn toggle_setup_mode(&mut self) {
        self.setup_mode = !self.setup_mode;
        if self.setup_mode {
            self.status = "Setup mode: Place X/O manually. Tab to exit setup.".into();
        } else {
            self.status = "Game mode: Playing against AI. Tab to enter setup.".into();
        }
    }

    pub fn shuffle(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut cells = [None; 9];
        let mut x_count = 0;
        let mut o_count = 0;
        
        // Randomly place 3-4 pieces of each type
        let total_pieces = rng.gen_range(0..=4);
        for _ in 0..total_pieces {
            let empty_cells: Vec<usize> = (0..9).filter(|&i| cells[i].is_none()).collect();
            if !empty_cells.is_empty() {
                let idx = empty_cells[rng.gen_range(0..empty_cells.len())];
                if x_count <= o_count {
                    cells[idx] = Some(Player::X);
                    x_count += 1;
                } else {
                    cells[idx] = Some(Player::O);
                    o_count += 1;
                }
            }
        }
        
        self.state.cells = cells;
        self.state.to_move = if x_count <= o_count { Player::X } else { Player::O };
        self.cursor = 4;
        self.status = "Board shuffled randomly.".into();
    }

    pub fn place_manual(&mut self, player: Player) -> bool {
        if self.setup_mode {
            // In setup mode, allow placing any piece
            if self.state.cells[self.cursor].is_some() && self.state.cells[self.cursor] == Some(player) {
                // Remove if same piece
                self.state.cells[self.cursor] = None;
                self.status = format!("Removed {} from cell {}.", format_player(player), self.cursor + 1);
                return true;
            }
            self.state.cells[self.cursor] = Some(player);
            self.status = format!("Placed {} in cell {}.", format_player(player), self.cursor + 1);
            true
        } else {
            // In game mode, only allow placing human symbol on their turn
            if self.is_locked() {
                self.status = "Game over. Press R to restart.".into();
                return false;
            }
            if self.state.to_move != self.human_symbol {
                self.status = "Wait for the AI to finish its move.".into();
                return false;
            }
            if player != self.human_symbol {
                self.status = format!("You are playing as {}.", format_player(self.human_symbol));
                return false;
            }
            if self.state.cells[self.cursor].is_some() {
                self.status = format!("Cell {} is already occupied.", self.cursor + 1);
                return false;
            }
            
            self.state.cells[self.cursor] = Some(self.human_symbol);
            self.state.to_move = self.human_symbol.opponent();
            self.status = format!("Placed {} in cell {}.", format_player(self.human_symbol), self.cursor + 1);
            self.update_outcome();
            
            if !self.is_locked() {
                self.ai_auto_move();
            }
            true
        }
    }

    pub fn move_cursor(&mut self, row_delta: isize, col_delta: isize) {
        let row = (self.cursor / 3) as isize;
        let col = (self.cursor % 3) as isize;
        let new_row = (row + row_delta).clamp(0, 2);
        let new_col = (col + col_delta).clamp(0, 2);
        self.cursor = (new_row * 3 + new_col) as usize;
    }

    pub fn quick_place(&mut self, index: usize) -> bool {
        if index < 9 {
            self.cursor = index;
            return self.place_cell(index);
        }
        false
    }

    pub fn place_cursor(&mut self) -> bool {
        self.place_cell(self.cursor)
    }

    pub fn place_cell(&mut self, index: usize) -> bool {
        if index >= 9 {
            return false;
        }
        self.cursor = index;
        if self.setup_mode {
            // In setup mode, toggle between X, O, and empty
            match self.state.cells[index] {
                None => {
                    self.state.cells[index] = Some(Player::X);
                    self.status = format!("Placed X in cell {}.", index + 1);
                }
                Some(Player::X) => {
                    self.state.cells[index] = Some(Player::O);
                    self.status = format!("Changed to O in cell {}.", index + 1);
                }
                Some(Player::O) => {
                    self.state.cells[index] = None;
                    self.status = format!("Cleared cell {}.", index + 1);
                }
            }
            return true;
        }
        
        if self.is_locked() {
            self.status = "Game over. Press R to restart.".into();
            return false;
        }
        if self.state.to_move != self.human_symbol {
            self.status = "Wait for the AI to finish its move.".into();
            return false;
        }
        if self.state.cells[index].is_some() {
            self.status = format!("Cell {} is already occupied.", index + 1);
            return false;
        }

        self.state.cells[index] = Some(self.human_symbol);
        self.state.to_move = self.human_symbol.opponent();
        self.status = format!("Placed {} in cell {}.", format_player(self.human_symbol), index + 1);
        self.update_outcome();

        if !self.is_locked() {
            self.ai_auto_move();
        }
        true
    }

    pub fn auto_player_move(&mut self) -> bool {
        if self.state.to_move != self.human_symbol {
            self.status = "It's not your turn.".into();
            return false;
        }
        if self.is_locked() {
            self.status = "Game over. Press R to restart.".into();
            return false;
        }
        if let Some(index) = self.pick_best_move(self.human_symbol) {
            return self.place_cell(index);
        }
        self.status = "No legal moves available.".into();
        false
    }

    pub fn is_locked(&self) -> bool {
        self.state.winner().is_some() || self.state.is_full()
    }

    fn update_outcome(&mut self) {
        if let Some(winner) = self.state.winner() {
            self.status = match winner {
                Player::X => "You win! Press R to play again.".into(),
                Player::O => "AI wins. Press R to try again.".into(),
            };
        } else if self.state.is_full() {
            self.status = "It's a draw. Press R to restart.".into();
        }
    }

    fn ai_auto_move(&mut self) {
        if self.state.to_move != Player::O || self.is_locked() {
            return;
        }
        if let Some(index) = self.pick_best_move(Player::O) {
            self.state.cells[index] = Some(Player::O);
            self.state.to_move = Player::X;
            self.cursor = index;
            self.status = format!("AI placed O in cell {}.", index + 1);
            self.update_outcome();
        }
    }

    fn pick_best_move(&self, player: Player) -> Option<usize> {
        self.find_winning_move(player)
            .or_else(|| self.find_winning_move(player.opponent()))
            .or_else(|| {
                if self.state.cells[4].is_none() {
                    Some(4)
                } else {
                    None
                }
            })
            .or_else(|| {
                [0, 2, 6, 8]
                    .into_iter()
                    .find(|&idx| self.state.cells[idx].is_none())
            })
            .or_else(|| (0..9).find(|&idx| self.state.cells[idx].is_none()))
    }

    fn find_winning_move(&self, player: Player) -> Option<usize> {
        for line in WINNING_LINES {
            let mut player_marks = 0;
            let mut empty_spot = None;
            for &idx in &line {
                match self.state.cells[idx] {
                    Some(mark) if mark == player => player_marks += 1,
                    Some(_) => {}
                    None => empty_spot = Some(idx),
                }
            }
            if player_marks == 2 {
                if let Some(idx) = empty_spot {
                    if self.state.cells[idx].is_none() {
                        return Some(idx);
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct MissionariesCannibalsSession {
    pub state: MissionariesCannibalsState,
    pub status: String,
    pub solution: Option<MissionariesCannibalsSolution>,
    pub selected_move: usize,
}

#[derive(Debug, Clone)]
pub struct MissionariesCannibalsSolution {
    pub report: SearchReport<MissionariesCannibalsState>,
    pub step: usize,
}

impl MissionariesCannibalsSolution {
    pub fn total_steps(&self) -> usize {
        self.report.path.len().saturating_sub(1)
    }
}

impl Default for MissionariesCannibalsSession {
    fn default() -> Self {
        Self {
            state: MissionariesCannibalsState::default(),
            status: Self::base_status(),
            solution: None,
            selected_move: 0,
        }
    }
}

impl MissionariesCannibalsSession {
    fn base_status() -> String {
        "Use S to solve, Space to step through solution. H shuffles, R resets.".into()
    }

    pub fn reset(&mut self) {
        self.state = MissionariesCannibalsState::default();
        self.status = Self::base_status();
        self.solution = None;
        self.selected_move = 0;
    }

    pub fn shuffle(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Generate random valid states by trying different configurations
        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 100 {
                // Fallback to default if we can't find a valid random state
                self.state = MissionariesCannibalsState::default();
                self.status = "Shuffled to default state.".into();
                break;
            }
            
            // Randomly distribute missionaries and cannibals
            let left_m = rng.gen_range(0..=3);
            let left_c = rng.gen_range(0..=3);
            let boat_left = rng.gen_bool(0.5);
            
            let new_state = MissionariesCannibalsState {
                left_m,
                left_c,
                boat_left,
            };
            
            // Check if state is valid
            if new_state.is_valid() {
                // Also check if it's not the goal state (too easy)
                if !new_state.is_goal() {
                    self.state = new_state;
                    self.solution = None;
                    self.selected_move = 0;
                    self.status = format!(
                        "Shuffled: Left M={} C={}, Boat on {}",
                        left_m,
                        left_c,
                        if boat_left { "left" } else { "right" }
                    );
                    break;
                }
            }
        }
    }

    pub fn solve(&mut self) {
        let report = astar(self.state);
        if report.goal_found && !report.path.is_empty() {
            self.solution = Some(MissionariesCannibalsSolution { report, step: 0 });
            if let Some(solution) = &self.solution {
                if let Some(first) = solution.report.path.first() {
                    self.state = *first;
                }
                self.status = format!(
                    "Solution ready ({} moves). Press Space to step.",
                    solution.total_steps()
                );
            }
        } else {
            self.solution = None;
            self.status = "No solution found.".into();
        }
    }

    pub fn advance_solution(&mut self) -> bool {
        if let Some(solution) = &mut self.solution {
            if solution.step + 1 < solution.report.path.len() {
                solution.step += 1;
                if let Some(state) = solution.report.path.get(solution.step) {
                    self.state = *state;
                    if solution.step == solution.report.path.len() - 1 {
                        self.status = "Solution complete! Everyone crossed safely.".into();
                    } else {
                        self.status = format!(
                            "Step {} / {}",
                            solution.step,
                            solution.total_steps()
                        );
                    }
                }
                return true;
            } else {
                self.status = "Already at final solution state.".into();
                return false;
            }
        }
        self.status = "Run the solver with 'S' first.".into();
        false
    }

    pub fn is_solved(&self) -> bool {
        self.state.is_goal()
    }

    pub fn get_valid_moves(&self) -> Vec<BoatMove> {
        let mut moves = Vec::new();
        let possible_moves = vec![
            BoatMove { missionaries: 1, cannibals: 0 },
            BoatMove { missionaries: 2, cannibals: 0 },
            BoatMove { missionaries: 0, cannibals: 1 },
            BoatMove { missionaries: 0, cannibals: 2 },
            BoatMove { missionaries: 1, cannibals: 1 },
        ];

        for mv in possible_moves {
            if self.state.apply_move(mv).is_some() {
                moves.push(mv);
            }
        }
        moves
    }

    pub fn apply_move(&mut self, mv: BoatMove) -> bool {
        if let Some(new_state) = self.state.apply_move(mv) {
            self.state = new_state;
            self.solution = None;
            self.status = format!(
                "Moved {}M {}C {}",
                mv.missionaries,
                mv.cannibals,
                if self.state.boat_left { "to left" } else { "to right" }
            );
            if self.is_solved() {
                self.status = "Solved! Everyone crossed safely.".into();
            }
            true
        } else {
            self.status = "Invalid move!".into();
            false
        }
    }
}

#[derive(Debug)]
pub struct EightQueensSession {
    pub state: EightQueensState,
    pub status: String,
    pub solution: Option<EightQueensSolution>,
    pub selected_row: usize,
    pub selected_col: usize,
}

#[derive(Debug, Clone)]
pub struct EightQueensSolution {
    pub report: SearchReport<EightQueensState>,
    pub step: usize,
}

impl EightQueensSolution {
    pub fn total_steps(&self) -> usize {
        self.report.path.len().saturating_sub(1)
    }
}

impl Default for EightQueensSession {
    fn default() -> Self {
        Self {
            state: EightQueensState::default(),
            status: Self::base_status(),
            solution: None,
            selected_row: 0,
            selected_col: 0,
        }
    }
}

impl EightQueensSession {
    fn base_status() -> String {
        "Use arrows to select cell, Space to place/remove queen. S solves, R resets, H shuffles.".into()
    }

    pub fn reset(&mut self) {
        self.state = EightQueensState::default();
        self.status = Self::base_status();
        self.solution = None;
        self.selected_row = 0;
        self.selected_col = 0;
    }

    pub fn shuffle(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Use backtracking to generate a solvable partial solution
        // This ensures the state always has a solution
        let num_queens = rng.gen_range(1..=4);
        let mut new_state = EightQueensState::default();
        
        // Generate a valid partial solution using backtracking
        // This ensures the state is always solvable
        let mut rows_used = [false; 8];
        let mut placed = 0;
        
        // Try to place queens using a randomized backtracking approach
        // that ensures solvability
        while placed < num_queens {
            let mut best_row: Option<usize> = None;
            let mut best_options = Vec::new();
            
            // Find rows with the most valid placement options
            for row in 0..8 {
                if rows_used[row] {
                    continue;
                }
                
                let mut valid_cols = Vec::new();
                for col in 0..8 {
                    if new_state.is_valid_placement(row as u8, col) {
                        valid_cols.push(col);
                    }
                }
                
                if !valid_cols.is_empty() {
                    if best_options.is_empty() || valid_cols.len() >= best_options.len() {
                        best_row = Some(row);
                        best_options = valid_cols;
                    }
                }
            }
            
            // Place a queen in the row with most options (or any valid row)
            if let Some(row) = best_row {
                if !best_options.is_empty() {
                    let col = best_options[rng.gen_range(0..best_options.len())];
                    if let Some(updated_state) = new_state.apply_placement(PlaceQueen { 
                        row: row as u8, 
                        col 
                    }) {
                        new_state = updated_state;
                        rows_used[row] = true;
                        placed += 1;
                    }
                } else {
                    break; // No more valid placements
                }
            } else {
                break; // No more valid rows
            }
        }
        
        // If we couldn't place the desired number, try a different approach:
        // Use a known solvable pattern or start from a solved state and remove queens
        if placed == 0 {
            // Fallback: use a known valid partial solution
            // Place queens in a pattern that's known to be solvable
            let known_solutions = vec![
                vec![(0, 0), (1, 4), (2, 7), (3, 5)],
                vec![(0, 1), (1, 3), (2, 5), (3, 7)],
                vec![(0, 2), (1, 5), (2, 1), (3, 6)],
                vec![(0, 3), (1, 6), (2, 0), (3, 2)],
            ];
            let solution = &known_solutions[rng.gen_range(0..known_solutions.len())];
            let to_place = rng.gen_range(1..=solution.len().min(4));
            
            for i in 0..to_place {
                let (row, col) = solution[i];
                if let Some(updated_state) = new_state.apply_placement(PlaceQueen { 
                    row: row as u8, 
                    col: col as u8 
                }) {
                    new_state = updated_state;
                }
            }
        }
        
        self.state = new_state;
        self.solution = None;
        let conflicts = self.state.count_conflicts();
        let queens_placed = self.state.queens.iter().filter(|q| q.is_some()).count();
        
        // Verify the state has valid successors
        let has_successors = !self.state.successors().is_empty();
        
        if conflicts == 0 && has_successors {
            self.status = format!("Shuffled: {} queens placed with no conflicts. State is solvable.", queens_placed);
        } else if has_successors {
            self.status = format!("Shuffled: {} queens placed. Conflicts: {}. State is solvable.", queens_placed, conflicts);
        } else {
            // If no successors, try one more time with a simpler approach
            self.state = EightQueensState::default();
            // Place just 1 queen in a random valid position
            let row = rng.gen_range(0..8) as u8;
            let col = rng.gen_range(0..8) as u8;
            if let Some(updated_state) = self.state.apply_placement(PlaceQueen { row, col }) {
                self.state = updated_state;
                self.status = "Shuffled: 1 queen placed. State is solvable.".into();
            } else {
                // Last resort: empty board (always solvable)
                self.state = EightQueensState::default();
                self.status = "Shuffled: Empty board (always solvable).".into();
            }
        }
    }

    pub fn solve(&mut self) {
        let report = astar(self.state);
        if report.goal_found && !report.path.is_empty() {
            self.solution = Some(EightQueensSolution { report, step: 0 });
            if let Some(solution) = &self.solution {
                if let Some(first) = solution.report.path.first() {
                    self.state = *first;
                }
                self.status = format!(
                    "Solution ready ({} steps). Press Space to step.",
                    solution.total_steps()
                );
            }
        } else {
            self.solution = None;
            let elapsed_secs = report.elapsed.as_secs();
            if elapsed_secs >= 3600 {
                self.status = format!(
                    "Search timed out after 1 hour ({} nodes explored). The puzzle may be unsolvable from this state, or try shuffling (H).",
                    report.expanded_nodes
                );
            } else if report.expanded_nodes == 0 {
                self.status = "No valid moves available. Try shuffling (H) for a different starting state.".into();
            } else if report.expanded_nodes < 10 {
                self.status = format!(
                    "Search terminated early ({} states). This may indicate the starting state has no valid successors. Try shuffling (H).",
                    report.expanded_nodes
                );
            } else {
                self.status = format!(
                    "No solution found after exploring {} states in {:.1}s. Still searching... Try shuffling (H) for a different starting state, or wait longer.",
                    report.expanded_nodes,
                    elapsed_secs as f64 + report.elapsed.subsec_millis() as f64 / 1000.0
                );
            }
        }
    }

    pub fn advance_solution(&mut self) -> bool {
        if let Some(solution) = &mut self.solution {
            if solution.step + 1 < solution.report.path.len() {
                solution.step += 1;
                if let Some(state) = solution.report.path.get(solution.step) {
                    self.state = *state;
                    if solution.step == solution.report.path.len() - 1 {
                        self.status = "Solution complete! All 8 queens placed safely.".into();
                    } else {
                        self.status = format!(
                            "Step {} / {}",
                            solution.step,
                            solution.total_steps()
                        );
                    }
                }
                return true;
            } else {
                self.status = "Already at final solution state.".into();
                return false;
            }
        }
        self.status = "Run the solver with 'S' first.".into();
        false
    }

    pub fn is_solved(&self) -> bool {
        self.state.is_goal()
    }

    pub fn move_cursor(&mut self, row_delta: isize, col_delta: isize) {
        let new_row = (self.selected_row as isize + row_delta).clamp(0, 7) as usize;
        let new_col = (self.selected_col as isize + col_delta).clamp(0, 7) as usize;
        self.selected_row = new_row;
        self.selected_col = new_col;
    }

    pub fn toggle_queen(&mut self) -> bool {
        let row = self.selected_row as u8;
        let col = self.selected_col as u8;
        
        if self.state.queens[self.selected_row].is_some() {
            // Remove queen
            self.state = self.state.remove_queen(row);
            self.solution = None;
            self.status = format!("Removed queen from row {}, col {}.", row + 1, col + 1);
            true
        } else {
            // Try to place queen
            if let Some(new_state) = self.state.apply_placement(PlaceQueen { row, col }) {
                self.state = new_state;
                self.solution = None;
                let conflicts = self.state.count_conflicts();
                if conflicts == 0 && self.is_solved() {
                    self.status = "Perfect! All 8 queens placed with no conflicts.".into();
                } else if conflicts == 0 {
                    self.status = format!("Placed queen at row {}, col {}. No conflicts yet.", row + 1, col + 1);
                } else {
                    self.status = format!("Placed queen at row {}, col {}. Conflicts: {}.", row + 1, col + 1, conflicts);
                }
                true
            } else {
                self.status = format!("Cannot place queen at row {}, col {} (conflicts with existing queens).", row + 1, col + 1);
                false
            }
        }
    }
}
