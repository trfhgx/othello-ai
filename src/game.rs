use crate::board::{Board, Cell};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn to_cell(&self) -> Cell {
        match self {
            Player::Black => Cell::Black,
            Player::White => Cell::White,
        }
    }

    pub fn opponent(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GameState {
    Playing,
    BlackWin,
    WhiteWin,
    Draw,
}

pub struct Game {
    pub board: Board,
    pub current_player: Player,
    pub state: GameState,
    pub black_score: usize,
    pub white_score: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            board: Board::new(),
            current_player: Player::Black,
            state: GameState::Playing,
            black_score: 2,
            white_score: 2,
        };
        game.update_scores();
        game
    }

    pub fn make_move(&mut self, row: isize, col: isize) -> bool {
        if self.state != GameState::Playing {
            return false;
        }

        let color = self.current_player.to_cell();
        if !self.board.valid_move(row, col, color) {
            return false;
        }

        self.board.render_move(row, col, color);
        self.update_scores();
        self.switch_player();
        self.check_game_end();
        true
    }

    fn switch_player(&mut self) {
        let next_player = self.current_player.opponent();
        let moves = self.board.all_valid_moves(next_player.to_cell());

        if moves.is_empty() {
            let current_moves = self.board.all_valid_moves(self.current_player.to_cell());
            if current_moves.is_empty() {
                self.state = self.determine_winner();
            }
        } else {
            self.current_player = next_player;
        }
    }

    fn update_scores(&mut self) {
        let mut black = 0;
        let mut white = 0;

        for row in &self.board.matrix {
            for cell in row {
                match cell {
                    Cell::Black => black += 1,
                    Cell::White => white += 1,
                    Cell::Empty => {}
                }
            }
        }

        self.black_score = black;
        self.white_score = white;
    }

    fn check_game_end(&mut self) {
        let black_moves = self.board.all_valid_moves(Cell::Black);
        let white_moves = self.board.all_valid_moves(Cell::White);

        if black_moves.is_empty() && white_moves.is_empty() {
            self.state = self.determine_winner();
        }
    }

    fn determine_winner(&self) -> GameState {
        match self.black_score.cmp(&self.white_score) {
            std::cmp::Ordering::Greater => GameState::BlackWin,
            std::cmp::Ordering::Less => GameState::WhiteWin,
            std::cmp::Ordering::Equal => GameState::Draw,
        }
    }

    pub fn reset(&mut self) {
        *self = Game::new();
    }
}