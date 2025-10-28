# Othello AI

A desktop implementation of Othello (Reversi) built with Rust and egui, featuring an AI opponent powered by minimax algorithm.

## Overview

Reversito is a strategic board game where players take turns placing discs on an 8x8 grid. The objective is to flip opponent discs by trapping them between your own pieces. The player with the most discs of their color when no more moves are available wins.

## Features

- **Interactive GUI**: Clean, responsive interface built with egui/eframe
- **Visual Move Indicators**: Valid moves are highlighted during gameplay
- **Score Tracking**: Real-time display of black and white piece counts
- **AI Opponent**: Play against a computer opponent using minimax search
- **Game State Management**: Automatic turn switching, win/loss/draw detection
- **New Game**: Reset functionality to start fresh games

## Technical Stack

- **Language**: Rust (2024 edition)
- **GUI Framework**: egui 0.33.0 / eframe 0.33.0
- **Architecture**: Modular design with separate board logic, game state, and AI modules

## Project Structure

```
src/
├── main.rs       # Application entry point and GUI implementation
├── lib.rs        # Library exports
├── board.rs      # Board representation and move validation logic
├── game.rs       # Game state management and player logic
└── ai.rs         # Minimax algorithm implementation (in development)
```

## Building and Running

### Prerequisites

- Rust toolchain (2024 edition or later)
- Cargo package manager

### Commands

```bash

# Run the application
cargo run --release
```

## Gameplay

1. Launch the application to see the standard Othello starting position
2. Click on highlighted cells to place your disc
3. Valid moves show semi-transparent indicators
4. Click "LET AI PLAY" to enable the AI opponent for the current player
5. Use "New Game" to reset the board


## Development Status

The core game mechanics are fully implemented and functional. AI opponent functionality is under development, with the minimax algorithm structure in place.

## License

This project is open source and available for educational and personal use.
