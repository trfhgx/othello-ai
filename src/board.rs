#[derive(PartialEq, Copy, Clone, Debug)]
enum Cell {
    Empty,
    Black,
    White,
}

struct Board {
    matrix: [[Cell ;8] ;8]
}

impl Board {
    fn new() -> Self {
        let mut board = Board {
            matrix: [[Cell::Empty; 8]; 8]

        };
        board.matrix[3][3] = Cell::White;
        board.matrix[3][4] = Cell::Black;
        board.matrix[4][3] = Cell::Black;
        board.matrix[4][4] = Cell::White;
        board
    }
    // here ndiro to check if a specfic move is valid or not
    // for a valid move in any direction of a straight line the first idsk on any opposing end must be the reverse
    //
    fn valid_move(&self, row: isize, col: isize, color: Cell) -> bool {
        if !(self.matrix[row as usize][col as usize] == Cell::Empty) {
            return false;
        }

        let opposite_cell = if color == Cell::Black {
            Cell::White
        } else {
            Cell::Black
        };

        let len: isize = self.matrix.len() as isize;
        let mut flip = -1;

        for i in 0..8 {
            flip = flip * -1;
            let (mut row_add, mut col_add) = match i {
                0..2 => (0, 1),
                2..4 => (1, 0),
                4..6 => (1, 1),
                6..8 => (1, -1),
                _ => (0, 0),
            };
            row_add = row_add * flip;
            col_add = col_add * flip;
            // Check for out of bounds
            let new_row = row + row_add;
            let new_col = col + col_add;
            if new_row < 0 || new_col < 0 || new_row >= len || new_col >= len {
                continue;
            }
            if self.matrix[(row + row_add) as usize][(col + col_add) as usize] == opposite_cell {
                let (mut i, mut u) = (row, col);
                while i < len && u < len && i >= 0 && u >= 0 {
                    i += row_add;
                    u += col_add;

                    if i < 0 || u < 0 || i >= len || u >= len {
                        break;
                    }

                    if self.matrix[i as usize][u as usize] == Cell::Empty {
                        break;
                    }
                    if self.matrix[i as usize][u as usize] == color {
                        return true;
                    }
                }
            }
        }
        false
    }

}