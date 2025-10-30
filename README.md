# Othello AI

Desktop implementation of Othello (Reversi) with AI opponent using minimax algorithm with alpha-beta pruning.

## Features

- Interactive GUI with move indicators
- AI opponent with adjustable difficulty (depth 1-10)
- Multiple game modes: Human vs Human, Human vs AI, AI vs AI
- Sound effects for moves and game outcomes
- Real-time score tracking

## Controls

- **Click**: Place disc / Select options
- **↑/↓ Arrow Keys**: Adjust AI difficulty

## Building

```bash
cargo run --release
```

## Technical Stack

- Rust 2024 edition
- egui/eframe 0.33.0
- rodio 0.17
- Minimax with alpha-beta pruning

## Game Rules

Black moves first. Place discs to trap opponent pieces in straight lines. Trapped pieces flip to your color. Game ends when no moves available. Most discs wins.

## License

Open source for educational and personal use.
