╔══════════════════════════════════════════════════════════════════╗
║          AI Puzzle Suite - Terminal Application (TUI)            ║
║                    Developed by Adel Enazi                        ║
║                         Version 1.0                               ║
╚══════════════════════════════════════════════════════════════════╝

REQUIREMENTS:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
• Windows 10 or later
• Command Prompt or PowerShell
• Terminal must support:
  - ANSI color codes (Windows 10+ supports this)
  - Unicode characters (box-drawing characters)
  - Raw input mode

HOW TO RUN:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Method 1 - Double-click "Run AI Puzzle Suite.bat"
  (Easiest method - opens Command Prompt automatically)

Method 2 - From Command Prompt:
  1. Open Command Prompt (cmd.exe) or PowerShell
  2. Navigate to this folder:
     cd "C:\path\to\ai-puzzle-suite-tui-windows"
  3. Run the application:
     ai-puzzle-suite-tui.exe

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
  • Make sure you're using Command Prompt or PowerShell
  • Try running from Command Prompt manually (Method 2)
  • Check Windows Defender hasn't blocked the file
  • Right-click → Properties → Unblock (if available)

If colors/characters look wrong:
  • Ensure you're using Windows 10 or later
  • Use Command Prompt (cmd.exe) for best compatibility
  • PowerShell also works but may have minor display differences
  • Check that code page is set to UTF-8:
    chcp 65001

If you get "Windows protected your PC":
  • Click "More info"
  • Click "Run anyway"
  • This is normal for unsigned applications

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

