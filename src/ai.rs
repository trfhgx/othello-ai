use crate::game::Game;
use crate::board::Cords;
use crate::board::Cell;


//  if ismax true then we'll assume we'll playing as X otherwise O

pub fn eval(_game: &Game) -> isize {
    let mut score: isize = 0;

    for row in 0..8 {
        for col in 0..8 {
            match _game.board.matrix[row][col] {
                crate::board::Cell::Black => score += 1,
                crate::board::Cell::White => score -= 1,
                _ => {}
            }
        }
    }

    score += (_game.black_score as isize - _game.white_score as isize) * 10;



    score


}
pub fn minimax(_game: &Game, color: Cell, depth:usize,max_depth: usize) -> (isize, Option<Cords>) {
    if depth > max_depth {
        return (eval(_game), None);
    }


    let moves = _game.board.all_valid_moves(color);
    let mut best_score = if color == Cell::Black { -f32::INFINITY } else { f32::INFINITY };
    let mut best_move = None;

    for index in moves {
        let mut new_game: Game = _game.clone();
        new_game.make_move(index.0, index.1);

        let opposite_color = match color {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => color,
        };
        let (score, _) = minimax(&new_game, opposite_color, depth+1, max_depth);

        if (color == Cell::Black && (score as f32) > best_score) || (color == Cell::White && (score as f32) < best_score) {
            best_score = score as f32;
            best_move = Some(index);
        }
    }

    (best_score as isize, best_move)
}