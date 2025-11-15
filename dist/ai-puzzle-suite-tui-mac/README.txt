╔══════════════════════════════════════════════════════════════════╗
║          AI Puzzle Suite - Terminal Application (TUI)            ║
║                    Developed by Adel Enazi                        ║
║                         Version 1.0                               ║
╚══════════════════════════════════════════════════════════════════╝

REQUIREMENTS:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
• macOS 10.13 or later
• Terminal application (Terminal.app - included with macOS)
• Terminal must support:
  - ANSI color codes
  - Unicode characters (box-drawing characters)
  - Raw input mode

HOW TO RUN:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Method 1 - Double-click the "Run AI Puzzle Suite.command" file
  (Easiest method - opens Terminal automatically)

Method 2 - From Terminal:
  1. Open Terminal.app
  2. Navigate to this folder:
     cd /path/to/ai-puzzle-suite-tui-mac
  3. Run the application:
     ./ai-puzzle-suite-tui

BASIC CONTROLS:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Main Menu:
  ↑↓     Navigate puzzle list
  Enter  Select puzzle
  Q      Quit application

In Puzzles:
  See puzzle-specific instructions at the bottom of each screen
  B      Back to main menu
  Q      Quit application

PUZZLES INCLUDED:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. 8-Puzzle Solver
   - Slide tiles to solve using A* algorithm
   - Manhattan distance heuristic
   - Edit goal state, shuffle boards

2. XOR Tic-Tac-Toe
   - Strategic game variant
   - Setup mode and game mode
   - A* hints available

3. Missionaries & Cannibals
   - Classic river crossing puzzle
   - A* finds optimal solution
   - Step through solutions

4. 8 Queens Problem
   - Place queens without conflicts
   - Constraint satisfaction
   - Watch A* solve it

5. About This Program
   - Learn about the application
   - A* algorithm explanation
   - Acknowledgments

TROUBLESHOOTING:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
If the app doesn't run:
  • Make sure Terminal.app is your default terminal
  • Try running from Terminal manually (Method 2)
  • Check that the file has execute permissions:
    chmod +x ai-puzzle-suite-tui

If colors/characters look wrong:
  • Ensure your terminal supports ANSI colors
  • Try using Terminal.app (not iTerm2 or other terminals)
  • Check terminal encoding is set to UTF-8

If you get "permission denied":
  • Right-click the binary → Get Info
  • Uncheck "Quarantine" if present
  • Or run: xattr -d com.apple.quarantine ai-puzzle-suite-tui

CREDITS:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Developed by: Adel Enazi

Special thanks to Professor Abdulrahman Fakki for teaching the
Artificial Intelligence course that inspired this project.

This application demonstrates the A* (A-Star) search algorithm
through interactive puzzle solving.

For more information, select "About This Program" from the main menu.

═══════════════════════════════════════════════════════════════════
Enjoy exploring AI algorithms through interactive puzzles!
═══════════════════════════════════════════════════════════════════

