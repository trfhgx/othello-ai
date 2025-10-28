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
pub fn minimax(_game: &Game, color: Cell, depth: usize, max_depth: usize, mut alpha: isize, mut beta: isize) -> (isize, Option<Cords>) {
    if depth > max_depth {
        return (eval(_game), None);
    }

    let moves = _game.board.all_valid_moves(color);

    if moves.is_empty() {
        return (eval(_game), None);
    }

    let mut best_score = if color == Cell::Black { isize::MIN } else { isize::MAX };
    let mut best_move = None;

    for index in moves {
        let mut new_game: Game = _game.clone();
        new_game.make_move(index.0, index.1);

        let opposite_color = match color {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => color,
        };

        let (score, _) = minimax(&new_game, opposite_color, depth + 1, max_depth, alpha, beta);

        if color == Cell::Black {
            if score > best_score {
                best_score = score;
                best_move = Some(index);
            }
            alpha = alpha.max(best_score);
            if beta <= alpha {
                break; // Beta cutoff
            }
        } else {
            if score < best_score {
                best_score = score;
                best_move = Some(index);
            }
            beta = beta.min(best_score);
            if beta <= alpha {
                break; // Alpha cutoff
            }
        }
    }

    (best_score, best_move)
}
