use std::io;
use std::time::Duration;

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::{
    app::{App, AppRoute, XorTicTacToeSession},
    puzzles::{EightPuzzleState, EightQueensState, MissionariesCannibalsState, Player, PuzzleId},
    search::SearchState,
};

pub fn run(app: &mut App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut menu_state = MenuState::default();

    while !app.should_exit() {
        terminal.draw(|frame| match app.route {
            AppRoute::MainMenu => render_main_menu(frame, app, &menu_state),
            AppRoute::Puzzle(id) => render_puzzle_shell(frame, app, id),
            AppRoute::Quit => {}
        })?;

        if let Some(event) = poll_event()? {
            match app.route {
                AppRoute::MainMenu => handle_main_menu_input(event, app, &mut menu_state),
                AppRoute::Puzzle(id) => handle_puzzle_input(event, app, id),
                AppRoute::Quit => break,
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn poll_event() -> Result<Option<Event>> {
    if event::poll(Duration::from_millis(50))? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}

#[derive(Default)]
struct MenuState {
    selected: usize,
}

fn handle_main_menu_input(event: Event, app: &mut App, menu_state: &mut MenuState) {
    if let Event::Key(KeyEvent {
        code,
        kind: KeyEventKind::Press,
        ..
    }) = event
    {
        match code {
            KeyCode::Char('q') | KeyCode::Char('Q') => app.request_quit(),
            KeyCode::Up => {
                if menu_state.selected > 0 {
                    menu_state.selected -= 1;
                }
            }
            KeyCode::Down => {
                if menu_state.selected + 1 < app.registry.descriptors.len() {
                    menu_state.selected += 1;
                }
            }
            KeyCode::Enter => {
                if let Some(descriptor) = app.registry.descriptors.get(menu_state.selected) {
                    app.select_puzzle(descriptor.id);
                }
            }
            KeyCode::Char(digit) if digit.is_ascii_digit() => {
                let index = digit.to_string().parse::<usize>().ok();
                if let Some(idx) = index.and_then(|n| n.checked_sub(1)) {
                    if idx < app.registry.descriptors.len() {
                        menu_state.selected = idx;
                        if let Some(descriptor) = app.registry.descriptors.get(menu_state.selected)
                        {
                            app.select_puzzle(descriptor.id);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn handle_puzzle_input(event: Event, app: &mut App, puzzle_id: PuzzleId) {
    if let Event::Key(KeyEvent {
        code,
        kind: KeyEventKind::Press,
        ..
    }) = event
    {
        match code {
            KeyCode::Esc | KeyCode::Char('b') | KeyCode::Char('B') => {
                app.select_main_menu();
                return;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                app.request_quit();
                return;
            }
            _ => {}
        }

        match puzzle_id {
            PuzzleId::EightPuzzle => handle_eight_puzzle_key(code, app),
            PuzzleId::XorTicTacToe => handle_xor_ttt_key(code, app),
            PuzzleId::MissionariesCannibals => handle_missionaries_cannibals_key(code, app),
            PuzzleId::EightQueens => handle_eight_queens_key(code, app),
            PuzzleId::About => {
                // About page only needs back/quit, handled by common keys above
            }
        }
    }
}

fn handle_eight_puzzle_key(code: KeyCode, app: &mut App) {
    match code {
        KeyCode::Tab => app.eight_puzzle.toggle_editing_goal(),
        KeyCode::Char('r') | KeyCode::Char('R') => app.eight_puzzle.reset(),
        KeyCode::Char('n') | KeyCode::Char('N') => app.eight_puzzle.new_board(),
        KeyCode::Char('h') | KeyCode::Char('H') => app.eight_puzzle.shuffle(),
        KeyCode::Char('s') | KeyCode::Char('S') => app.eight_puzzle.solve_current(),
        KeyCode::Char(' ') | KeyCode::Enter => {
            app.eight_puzzle.advance_solution();
        }
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            app.eight_puzzle.move_cursor(-1, 0);
        }
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
            app.eight_puzzle.move_cursor(1, 0);
        }
        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
            app.eight_puzzle.move_cursor(0, -1);
        }
        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
            app.eight_puzzle.move_cursor(0, 1);
        }
        KeyCode::Char(digit) if digit.is_ascii_digit() => {
            if let Some(num) = digit.to_digit(10) {
                if num >= 1 && num <= 8 {
                    app.eight_puzzle.place_number(num as u8);
                }
            }
        }
        _ => {}
    }
}

fn handle_xor_ttt_key(code: KeyCode, app: &mut App) {
    match code {
        KeyCode::Tab => app.xor_ttt.toggle_setup_mode(),
        KeyCode::Char('r') | KeyCode::Char('R') => app.xor_ttt.reset(),
        KeyCode::Char('h') | KeyCode::Char('H') => app.xor_ttt.shuffle(),
        KeyCode::Char('s') | KeyCode::Char('S') => {
            if app.xor_ttt.setup_mode {
                // In setup mode, S doesn't make sense
                app.xor_ttt.status = "Exit setup mode (Tab) to use auto-move.".into();
            } else {
                app.xor_ttt.auto_player_move();
            }
        }
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => app.xor_ttt.move_cursor(-1, 0),
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => app.xor_ttt.move_cursor(1, 0),
        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => app.xor_ttt.move_cursor(0, -1),
        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => app.xor_ttt.move_cursor(0, 1),
        KeyCode::Char('x') | KeyCode::Char('X') => {
            app.xor_ttt.place_manual(Player::X);
        }
        KeyCode::Char('o') | KeyCode::Char('O') => {
            app.xor_ttt.place_manual(Player::O);
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.xor_ttt.place_cursor();
        }
        KeyCode::Char(digit) if digit.is_ascii_digit() => {
            if let Some(index) = digit_to_index(digit) {
                app.xor_ttt.quick_place(index);
            }
        }
        _ => {}
    }
}

fn handle_missionaries_cannibals_key(code: KeyCode, app: &mut App) {
    match code {
        KeyCode::Char('r') | KeyCode::Char('R') => app.missionaries_cannibals.reset(),
        KeyCode::Char('h') | KeyCode::Char('H') => app.missionaries_cannibals.shuffle(),
        KeyCode::Char('s') | KeyCode::Char('S') => app.missionaries_cannibals.solve(),
        KeyCode::Char(' ') | KeyCode::Enter => {
            app.missionaries_cannibals.advance_solution();
        }
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            let moves = app.missionaries_cannibals.get_valid_moves();
            if !moves.is_empty() {
                app.missionaries_cannibals.selected_move = app.missionaries_cannibals.selected_move.saturating_sub(1);
                if app.missionaries_cannibals.selected_move >= moves.len() {
                    app.missionaries_cannibals.selected_move = moves.len().saturating_sub(1);
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
            let moves = app.missionaries_cannibals.get_valid_moves();
            if !moves.is_empty() {
                app.missionaries_cannibals.selected_move = (app.missionaries_cannibals.selected_move + 1).min(moves.len().saturating_sub(1));
            }
        }
        KeyCode::Char('1') => {
            let moves = app.missionaries_cannibals.get_valid_moves();
            if moves.len() > 0 {
                app.missionaries_cannibals.apply_move(moves[0]);
            }
        }
        KeyCode::Char('2') => {
            let moves = app.missionaries_cannibals.get_valid_moves();
            if moves.len() > 1 {
                app.missionaries_cannibals.apply_move(moves[1]);
            }
        }
        KeyCode::Char('3') => {
            let moves = app.missionaries_cannibals.get_valid_moves();
            if moves.len() > 2 {
                app.missionaries_cannibals.apply_move(moves[2]);
            }
        }
        KeyCode::Char('4') => {
            let moves = app.missionaries_cannibals.get_valid_moves();
            if moves.len() > 3 {
                app.missionaries_cannibals.apply_move(moves[3]);
            }
        }
        KeyCode::Char('5') => {
            let moves = app.missionaries_cannibals.get_valid_moves();
            if moves.len() > 4 {
                app.missionaries_cannibals.apply_move(moves[4]);
            }
        }
        _ => {}
    }
}

fn render_main_menu(frame: &mut Frame, app: &App, menu_state: &MenuState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(frame.size());

    let title = Paragraph::new("AI Puzzle Suite (TUI)")
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(title, layout[0]);

    let list_items: Vec<ListItem> = app
        .registry
        .descriptors
        .iter()
        .enumerate()
        .map(|(idx, descriptor)| {
            let prefix = format!("{}. {}", idx + 1, descriptor.name);
            ListItem::new(Line::from(vec![Span::raw(prefix)]))
        })
        .collect();
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(menu_state.selected));

    let list = List::new(list_items)
        .block(Block::default().title("Puzzles").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    frame.render_stateful_widget(list, layout[1], &mut list_state);

    if let Some(current) = app.registry.descriptors.get(menu_state.selected) {
        let details_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(layout[2]);
        
        let details = Paragraph::new(current.summary)
            .block(Block::default().title("Description").borders(Borders::ALL));
        frame.render_widget(details, details_area[0]);
        
        // Add author name
        let footer = Paragraph::new("Adel Enazi")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM));
        frame.render_widget(footer, details_area[1]);
    }
}

fn render_puzzle_shell(frame: &mut Frame, app: &App, puzzle_id: PuzzleId) {
    match puzzle_id {
        PuzzleId::EightPuzzle => render_eight_puzzle(frame, app),
        PuzzleId::XorTicTacToe => render_xor_ttt(frame, app),
        PuzzleId::MissionariesCannibals => render_missionaries_cannibals(frame, app),
        PuzzleId::EightQueens => render_eight_queens(frame, app),
        PuzzleId::About => render_about(frame, app),
    }
}

fn render_eight_puzzle(frame: &mut Frame, app: &App) {
    let descriptor = app.registry.descriptor(PuzzleId::EightPuzzle);
    let title = descriptor.map(|d| d.name).unwrap_or("8-Puzzle Solver");
    let summary = descriptor.map(|d| d.summary).unwrap_or("");
    let session = &app.eight_puzzle;

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(4),
        ])
        .split(frame.size());

    let header = Paragraph::new(format!(
        "{} — {}",
        title,
        if session.is_solved() {
            "Solved"
        } else {
            "In progress"
        }
    ))
    .alignment(Alignment::Center)
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, outer[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[1]);

    let board_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(15), Constraint::Length(8)])
        .split(body[0]);

    let current_selection = if session.editing_goal { 10 } else { session.selected_cell };
    let board_lines = render_eight_puzzle_board(&session.current, current_selection);
    let board_title = if session.editing_goal {
        if session.is_solved() { "Current Board (Solved)" } else { "Current Board" }
    } else {
        if session.is_solved() { "Current Board (Solved) [EDITING]" } else { "Current Board [EDITING]" }
    };
    let board_block = Paragraph::new(board_lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(board_title)
                .borders(Borders::ALL),
        );
    frame.render_widget(board_block, board_area[0]);

    let goal_selection = if session.editing_goal { session.goal_selected_cell } else { 10 };
    let goal_lines = render_eight_puzzle_board(&session.goal_state, goal_selection);
    let goal_title = if session.editing_goal {
        "Goal Board [EDITING]"
    } else {
        "Goal Board"
    };
    let goal_block = Paragraph::new(goal_lines)
        .alignment(Alignment::Center)
        .block(Block::default().title(goal_title).borders(Borders::ALL));
    frame.render_widget(goal_block, board_area[1]);

    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Min(8),
            Constraint::Length(4),
        ])
        .split(body[1]);

    let summary_block = Paragraph::new(summary)
        .block(Block::default().title("Summary").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(summary_block, info_chunks[0]);

    let stats_text = format!(
        "Moves made: {}\nHeuristic: {}\nSolved: {}",
        session.moves_made,
        session.current.manhattan_distance(),
        if session.is_solved() { "Yes" } else { "No" }
    );
    let stats_block =
        Paragraph::new(stats_text).block(Block::default().title("State").borders(Borders::ALL));
    frame.render_widget(stats_block, info_chunks[1]);

    let solver_content = match &session.solution {
        Some(solution) => {
            let stats = format!(
                "Steps total: {}\nCurrent step: {}\nExpanded nodes: {}\nVisited states: {}\nElapsed: {}",
                solution.total_steps(),
                solution.step,
                solution.report.expanded_nodes,
                solution.report.visited_states,
                format_duration(solution.report.elapsed)
            );
            let explanation = format!(
                "\n\nA* Algorithm Explanation:\n\nA* finds the shortest path\nusing: f(n) = g(n) + h(n)\n\n• g(n) = actual moves\n  from start to here\n• h(n) = estimated moves\n  to goal (Manhattan)\n\nMetrics:\n• Expanded nodes: {}\n  States we fully explored\n  (checked all neighbors)\n\n• Visited states: {}\n  All states we've seen\n  (in queue + explored)",
                solution.report.expanded_nodes,
                solution.report.visited_states
            );
            format!("{}{}", stats, explanation)
        },
        None => "Press S to run the A* solver.\n\nA* Algorithm:\nFinds optimal paths using:\nf(n) = g(n) + h(n)\n\n• g(n) = actual cost\n  from start\n• h(n) = heuristic\n  (Manhattan distance)\n\nExpanded nodes: States\nwe fully explored.\nVisited states: All states\nwe've encountered.".into(),
    };
    let solver_block = Paragraph::new(solver_content)
        .block(Block::default().title("Solver").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(solver_block, info_chunks[2]);

    let status_block = Paragraph::new(session.status.as_str())
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(status_block, info_chunks[3]);

    let instructions_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(outer[2]);
    
    let instructions = Paragraph::new(
        "Controls: Tab switch boards • ←→↑↓ select cell • 1-8 place number • H shuffle current/goal • S solve • Space step • R reset • N new board • B back • Q quit",
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL))
    .wrap(Wrap { trim: true });
    frame.render_widget(instructions, instructions_area[0]);
    
    let footer = Paragraph::new("Adel Enazi")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM));
    frame.render_widget(footer, instructions_area[1]);
}

fn render_eight_puzzle_board(state: &EightPuzzleState, selected_cell: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    // Top border - each cell is 7 characters wide
    lines.push(Line::from("┌───────┬───────┬───────┐"));
    
    for row in 0..3 {
        let mut cell_spans = Vec::new();
        cell_spans.push(Span::raw("│"));
        
        for col in 0..3 {
            let idx = row * 3 + col;
            let tile = state.tiles[idx];
            let is_selected = idx == selected_cell;
            
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else if tile == 0 {
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM)
            } else {
                Style::default()
                    .fg(Color::White)
            };
            
            // Center the content in a 7-character wide cell
            let content = if tile == 0 {
                "       ".to_string() // 7 spaces for empty cell
            } else {
                // Center single-digit numbers: 3 spaces + number + 3 spaces = 7 chars
                format!("   {}   ", tile)
            };
            
            cell_spans.push(Span::styled(content, style));
            cell_spans.push(Span::raw("│"));
        }
        
        lines.push(Line::from(cell_spans));
        
        // Middle or bottom border
        if row < 2 {
            lines.push(Line::from("├───────┼───────┼───────┤"));
        } else {
            lines.push(Line::from("└───────┴───────┴───────┘"));
        }
    }
    
    lines
}

fn handle_eight_queens_key(code: KeyCode, app: &mut App) {
    match code {
        KeyCode::Char('r') | KeyCode::Char('R') => app.eight_queens.reset(),
        KeyCode::Char('h') | KeyCode::Char('H') => app.eight_queens.shuffle(),
        KeyCode::Char('s') | KeyCode::Char('S') => app.eight_queens.solve(),
        KeyCode::Char(' ') | KeyCode::Enter => {
            // If solution exists, step through it; otherwise toggle queen
            if app.eight_queens.solution.is_some() {
                app.eight_queens.advance_solution();
            } else {
                app.eight_queens.toggle_queen();
            }
        }
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            app.eight_queens.move_cursor(-1, 0);
        }
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
            app.eight_queens.move_cursor(1, 0);
        }
        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
            app.eight_queens.move_cursor(0, -1);
        }
        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
            app.eight_queens.move_cursor(0, 1);
        }
        _ => {}
    }
}

fn render_xor_ttt(frame: &mut Frame, app: &App) {
    let descriptor = app.registry.descriptor(PuzzleId::XorTicTacToe);
    let title = descriptor.map(|d| d.name).unwrap_or("XOR Tic-Tac-Toe");
    let summary = descriptor.map(|d| d.summary).unwrap_or("");
    let session = &app.xor_ttt;

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(4),
        ])
        .split(frame.size());

    let mode_indicator = if session.setup_mode {
        " [SETUP MODE]"
    } else if session.is_locked() {
        " [GAME OVER]"
    } else {
        " [PLAYING]"
    };
    
    let header = Paragraph::new(format!("{}{}", title, mode_indicator))
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, outer[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[1]);

    let board_lines = render_tic_tac_toe_board(session);
    let board_title = if session.setup_mode {
        "Board [SETUP MODE]"
    } else {
        "Board"
    };
    let board_block = Paragraph::new(board_lines)
        .alignment(Alignment::Center)
        .block(Block::default().title(board_title).borders(Borders::ALL));
    frame.render_widget(board_block, body[0]);

    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(5),
            Constraint::Min(5),
        ])
        .split(body[1]);

    let summary_block = Paragraph::new(summary)
        .block(Block::default().title("Summary").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(summary_block, info_chunks[0]);

    let info_text = format!(
        "Next player: {}\nCursor cell: {}\nWinner: {}\nBoard full: {}",
        format_player(session.state.to_move),
        session.cursor + 1,
        session.state.winner().map(format_player).unwrap_or("—"),
        if session.state.is_full() { "Yes" } else { "No" }
    );
    let info_block =
        Paragraph::new(info_text).block(Block::default().title("State").borders(Borders::ALL));
    frame.render_widget(info_block, info_chunks[1]);

    let status_block = Paragraph::new(session.status.as_str())
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(status_block, info_chunks[2]);

    let instructions_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(outer[2]);
    
    let instructions = Paragraph::new(
        "Controls: Tab setup mode • ←→↑↓ move cursor • X/O place pieces • 1-9 quick place • Space toggle • H shuffle • S auto-move • R restart • B back • Q quit",
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL))
    .wrap(Wrap { trim: true });
    frame.render_widget(instructions, instructions_area[0]);
    
    let footer = Paragraph::new("Adel Enazi")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM));
    frame.render_widget(footer, instructions_area[1]);
}

fn render_tic_tac_toe_board(session: &XorTicTacToeSession) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    // Top border
    lines.push(Line::from("┌───────┬───────┬───────┐"));
    
    for row in 0..3 {
        let mut cell_spans = Vec::new();
        cell_spans.push(Span::raw("│"));
        
        for col in 0..3 {
            let idx = row * 3 + col;
            let is_selected = session.cursor == idx;
            let cell_value = session.state.cells[idx];
            
            let symbol = match cell_value {
                Some(Player::X) => "X",
                Some(Player::O) => "O",
                None => "·",
            };
            
            let style = if is_selected {
                if cell_value.is_none() && (!session.is_locked() || session.setup_mode) {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                }
            } else if cell_value == Some(Player::O) {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if cell_value == Some(Player::X) {
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM)
            };
            
            // Center the symbol in a 7-character wide cell
            let content = format!("   {}   ", symbol);
            
            cell_spans.push(Span::styled(content, style));
            cell_spans.push(Span::raw("│"));
        }
        
        lines.push(Line::from(cell_spans));
        
        // Middle or bottom border
        if row < 2 {
            lines.push(Line::from("├───────┼───────┼───────┤"));
        } else {
            lines.push(Line::from("└───────┴───────┴───────┘"));
        }
    }
    
    lines
}

fn format_duration(duration: Duration) -> String {
    if duration.as_secs_f64() < 1.0 {
        format!("{:.2} ms", duration.as_secs_f64() * 1_000.0)
    } else {
        format!("{:.2} s", duration.as_secs_f64())
    }
}

fn format_player(player: Player) -> &'static str {
    match player {
        Player::X => "X",
        Player::O => "O",
    }
}

fn digit_to_index(ch: char) -> Option<usize> {
    ch.to_digit(10).and_then(|d| {
        if (1..=9).contains(&d) {
            Some((d - 1) as usize)
        } else {
            None
        }
    })
}

fn render_missionaries_cannibals(frame: &mut Frame, app: &App) {
    let descriptor = app.registry.descriptor(PuzzleId::MissionariesCannibals);
    let title = descriptor.map(|d| d.name).unwrap_or("Missionaries & Cannibals");
    let summary = descriptor.map(|d| d.summary).unwrap_or("");
    let session = &app.missionaries_cannibals;

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(4),
        ])
        .split(frame.size());

    let header = Paragraph::new(format!(
        "{} — {}",
        title,
        if session.is_solved() {
            "Solved"
        } else {
            "In progress"
        }
    ))
    .alignment(Alignment::Center)
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, outer[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[1]);

    let state_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(12), Constraint::Min(8)])
        .split(body[0]);

    let state_lines = render_mc_state(&session.state);
    let state_block = Paragraph::new(state_lines)
        .alignment(Alignment::Center)
        .block(Block::default().title("Current State").borders(Borders::ALL));
    frame.render_widget(state_block, state_area[0]);

    let valid_moves = session.get_valid_moves();
    let moves_text = if valid_moves.is_empty() {
        "No valid moves available.".into()
    } else {
        valid_moves
            .iter()
            .enumerate()
            .map(|(idx, mv)| {
                let marker = if idx == session.selected_move { ">" } else { " " };
                format!("{} {}. Move {}M {}C", marker, idx + 1, mv.missionaries, mv.cannibals)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let moves_block = Paragraph::new(moves_text)
        .block(Block::default().title("Valid Moves").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(moves_block, state_area[1]);

    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
            Constraint::Length(4),
        ])
        .split(body[1]);

    let summary_block = Paragraph::new(summary)
        .block(Block::default().title("Summary").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(summary_block, info_chunks[0]);

    let solver_text = match &session.solution {
        Some(solution) => {
            let stats = format!(
                "Steps total: {}\nCurrent step: {}\nExpanded nodes: {}\nVisited states: {}\nElapsed: {}",
                solution.total_steps(),
                solution.step,
                solution.report.expanded_nodes,
                solution.report.visited_states,
                format_duration(solution.report.elapsed)
            );
            let explanation = format!(
                "\n\nA* Algorithm Explanation:\n\nA* finds the shortest path\nusing: f(n) = g(n) + h(n)\n\n• g(n) = actual moves\n  from start to here\n• h(n) = estimated moves\n  to goal (people on left)\n\nMetrics:\n• Expanded nodes: {}\n  States we fully explored\n  (checked all neighbors)\n\n• Visited states: {}\n  All states we've seen\n  (in queue + explored)",
                solution.report.expanded_nodes,
                solution.report.visited_states
            );
            format!("{}{}", stats, explanation)
        },
        None => "Press S to run the A* solver.\n\nA* Algorithm:\nFinds optimal paths using:\nf(n) = g(n) + h(n)\n\n• g(n) = actual cost\n  from start\n• h(n) = heuristic\n  (people on left side)\n\nExpanded nodes: States\nwe fully explored.\nVisited states: All states\nwe've encountered.".into(),
    };
    let solver_block = Paragraph::new(solver_text)
        .block(Block::default().title("Solver").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(solver_block, info_chunks[1]);

    let status_block = Paragraph::new(session.status.as_str())
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(status_block, info_chunks[2]);

    let instructions_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(outer[2]);
    
    let instructions = Paragraph::new(
        "Controls: 1-5 select/apply move • ↑↓ navigate moves • S solve • Space step solution • H shuffle • R reset • B back • Q quit",
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL))
    .wrap(Wrap { trim: true });
    frame.render_widget(instructions, instructions_area[0]);
    
    let footer = Paragraph::new("Adel Enazi")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM));
    frame.render_widget(footer, instructions_area[1]);
}

fn render_mc_state(state: &MissionariesCannibalsState) -> Vec<Line<'static>> {
    let right_m = 3 - state.left_m;
    let right_c = 3 - state.left_c;
    
    let mut lines = Vec::new();
    
    // All lines are exactly 27 characters wide (including borders)
    // Content width is 25 (excluding │ on each side)
    
    // Top border
    lines.push(Line::from("┌─────────────────────────┐"));
    
    // Empty line
    lines.push(Line::from("│                         │"));
    
    // Left side - ensure exact width
    let boat_left_str = if state.boat_left { "🚤" } else { "  " };
    let left_content = format!("LEFT:  M={} C={} {}", state.left_m, state.left_c, boat_left_str);
    let left_padded = format!("│{:<25}│", left_content);
    lines.push(Line::from(left_padded));
    
    // Empty line
    lines.push(Line::from("│                         │"));
    
    // River separator
    lines.push(Line::from("│     ═══════════════     │"));
    
    // River label - centered
    let river_padded = format!("│{:^25}│", "RIVER");
    lines.push(Line::from(river_padded));
    
    // River separator
    lines.push(Line::from("│     ═══════════════     │"));
    
    // Empty line
    lines.push(Line::from("│                         │"));
    
    // Right side - ensure exact width
    let boat_right_str = if !state.boat_left { "🚤" } else { "  " };
    let right_content = format!("RIGHT: M={} C={} {}", right_m, right_c, boat_right_str);
    let right_padded = format!("│{:<25}│", right_content);
    lines.push(Line::from(right_padded));
    
    // Empty line
    lines.push(Line::from("│                         │"));
    
    // Bottom border
    lines.push(Line::from("└─────────────────────────┘"));
    
    lines
}

fn render_eight_queens(frame: &mut Frame, app: &App) {
    let descriptor = app.registry.descriptor(PuzzleId::EightQueens);
    let title = descriptor.map(|d| d.name).unwrap_or("8 Queens Problem");
    let summary = descriptor.map(|d| d.summary).unwrap_or("");
    let session = &app.eight_queens;

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(4),
        ])
        .split(frame.size());

    let header = Paragraph::new(format!(
        "{} — {}",
        title,
        if session.is_solved() {
            "Solved"
        } else {
            "In progress"
        }
    ))
    .alignment(Alignment::Center)
    .style(
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    )
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, outer[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[1]);

    let board_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(18), Constraint::Length(4)])
        .split(body[0]);

    let board_lines = render_queens_board(&session.state, session.selected_row, session.selected_col);
    let board_block = Paragraph::new(board_lines)
        .alignment(Alignment::Center)
        .block(Block::default().title("Chessboard").borders(Borders::ALL));
    frame.render_widget(board_block, board_area[0]);

    let stats_text = format!(
        "Queens placed: {}/8\nConflicts: {}\nHeuristic: {}",
        session.state.queens.iter().filter(|q| q.is_some()).count(),
        session.state.count_conflicts(),
        session.state.heuristic()
    );
    let stats_block = Paragraph::new(stats_text)
        .block(Block::default().title("State").borders(Borders::ALL));
    frame.render_widget(stats_block, board_area[1]);

    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
            Constraint::Length(4),
        ])
        .split(body[1]);

    let summary_block = Paragraph::new(summary)
        .block(Block::default().title("Summary").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(summary_block, info_chunks[0]);

    let solver_text = match &session.solution {
        Some(solution) => {
            let stats = format!(
                "Steps total: {}\nCurrent step: {}\nExpanded nodes: {}\nVisited states: {}\nElapsed: {}",
                solution.total_steps(),
                solution.step,
                solution.report.expanded_nodes,
                solution.report.visited_states,
                format_duration(solution.report.elapsed)
            );
            let explanation = format!(
                "\n\nA* Algorithm Explanation:\n\nA* finds the shortest path\nusing: f(n) = g(n) + h(n)\n\n• g(n) = actual moves\n  from start to here\n• h(n) = estimated cost\n  (conflicts + missing)\n\nMetrics:\n• Expanded nodes: {}\n  States we fully explored\n  (checked all neighbors)\n\n• Visited states: {}\n  All states we've seen\n  (in queue + explored)",
                solution.report.expanded_nodes,
                solution.report.visited_states
            );
            format!("{}{}", stats, explanation)
        },
        None => "Press S to run the A* solver.\n\nA* Algorithm:\nFinds optimal paths using:\nf(n) = g(n) + h(n)\n\n• g(n) = actual cost\n  from start\n• h(n) = heuristic\n  (conflicts + missing)\n\nExpanded nodes: States\nwe fully explored.\nVisited states: All states\nwe've encountered.".into(),
    };
    let solver_block = Paragraph::new(solver_text)
        .block(Block::default().title("Solver").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(solver_block, info_chunks[1]);

    let status_block = Paragraph::new(session.status.as_str())
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(status_block, info_chunks[2]);

    let instructions_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(outer[2]);
    
    let instructions = Paragraph::new(
        "Controls: ←→↑↓ select cell • Space place/remove queen • S solve • Space step solution • H shuffle • R reset • B back • Q quit",
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL))
    .wrap(Wrap { trim: true });
    frame.render_widget(instructions, instructions_area[0]);
    
    let footer = Paragraph::new("Adel Enazi")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM));
    frame.render_widget(footer, instructions_area[1]);
}

fn render_queens_board(state: &EightQueensState, selected_row: usize, selected_col: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    // Top border
    lines.push(Line::from("┌───────────────────────────────┐"));
    
    // Column numbers
    let mut col_header = String::from("│   ");
    for col in 0..8 {
        col_header.push_str(&format!("{} ", col + 1));
    }
    col_header.push_str("│");
    lines.push(Line::from(col_header));
    
    // Separator
    lines.push(Line::from("├───────────────────────────────┤"));
    
    for row in 0..8 {
        let mut row_spans = Vec::new();
        row_spans.push(Span::raw("│"));
        row_spans.push(Span::raw(format!("{} ", row + 1)));
        
        for col in 0..8 {
            let is_selected = selected_row == row && selected_col == col;
            let has_queen = state.queens[row].map(|q| q == col as u8).unwrap_or(false);
            
            let (symbol, style) = if has_queen {
                if is_selected {
                    ("♛", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD))
                } else {
                    ("♛", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                }
            } else {
                if is_selected {
                    ("·", Style::default().fg(Color::Black).bg(Color::Yellow))
                } else {
                    // Alternate colors for chessboard pattern
                    let is_light = (row + col) % 2 == 0;
                    if is_light {
                        ("·", Style::default().fg(Color::DarkGray))
                    } else {
                        ("·", Style::default().fg(Color::Gray))
                    }
                }
            };
            
            row_spans.push(Span::styled(format!("{} ", symbol), style));
        }
        
        row_spans.push(Span::raw("│"));
        lines.push(Line::from(row_spans));
    }
    
    // Bottom border
    lines.push(Line::from("└───────────────────────────────┘"));
    
    lines
}

fn render_about(frame: &mut Frame, app: &App) {
    let descriptor = app.registry.descriptor(PuzzleId::About);
    let title = descriptor.map(|d| d.name).unwrap_or("About This Program");

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(4),
        ])
        .split(frame.size());

    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, outer[0]);

    let body = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(8),
        ])
        .split(outer[1]);

    // Program Explanation
    let program_text = "AI Puzzle Suite (TUI)\n\n\
This interactive terminal application demonstrates the A* (A-Star) search algorithm \
through four classic AI puzzles:\n\n\
• 8-Puzzle: Slide tiles to solve using Manhattan distance heuristic\n\
• XOR Tic-Tac-Toe: Strategic game with A* hints\n\
• Missionaries & Cannibals: River crossing puzzle\n\
• 8 Queens: Constraint satisfaction problem\n\n\
Each puzzle showcases how A* efficiently finds optimal solutions by exploring \
the state space using the formula: f(n) = g(n) + h(n)\n\n\
• g(n) = actual cost from start to current state\n\
• h(n) = heuristic estimate to goal\n\
• f(n) = total estimated cost\n\n\
The algorithm expands nodes with the lowest f(n) first, ensuring optimal solutions \
when the heuristic is admissible (never overestimates).";
    
    let program_block = Paragraph::new(program_text)
        .block(Block::default().title("Program Overview").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(program_block, body[0]);

    // What's Happening
    let happening_text = "What's Happening?\n\n\
When you press 'S' to solve a puzzle, the A* algorithm:\n\n\
1. Starts from the current puzzle state\n\
2. Generates all valid successor states\n\
3. Calculates f(n) = g(n) + h(n) for each state\n\
4. Explores states with lowest f(n) first\n\
5. Continues until goal state is found\n\n\
The metrics shown:\n\
• Expanded Nodes: States fully explored (all neighbors checked)\n\
• Visited States: All states encountered (in queue + explored)\n\
• Elapsed Time: How long the search took\n\n\
You can step through the solution to see each move the algorithm found!";
    
    let happening_block = Paragraph::new(happening_text)
        .block(Block::default().title("How A* Works").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(happening_block, body[1]);

    // Acknowledgments
    let acknowledgments_text = "Acknowledgments\n\n\
Special thanks to Professor Abdulrahman Fakki for teaching the \
Artificial Intelligence course that inspired this project.\n\n\
This application was developed as a demonstration of search algorithms \
and their practical applications in solving puzzles and games.\n\n\
All regards,\n\
Adel Enazi";
    
    let acknowledgments_block = Paragraph::new(acknowledgments_text)
        .block(Block::default().title("Acknowledgments").borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);
    frame.render_widget(acknowledgments_block, body[2]);

    // Instructions
    let instructions_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(outer[2]);
    
    let instructions = Paragraph::new(
        "Controls: B back to menu • Q quit",
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL))
    .wrap(Wrap { trim: true });
    frame.render_widget(instructions, instructions_area[0]);
    
    let footer = Paragraph::new("Adel Enazi")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM));
    frame.render_widget(footer, instructions_area[1]);
}
