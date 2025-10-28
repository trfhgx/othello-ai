
pub type Vector = (isize, isize);
pub type Cords = (isize, isize);

pub type Mova = (Cords, Cords,Vector);
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Cell {
    Empty,
    Black,
    White,
}
#[derive(Clone)]

pub struct Board {
    pub matrix: [[Cell; 8]; 8],
}

pub struct Move {
    pub soli: Vec<Mova>,
}

impl Move {
    pub fn new() -> Self {
        Move {
            soli: Vec::new(),

        }
    }
}
impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            matrix: [[Cell::Empty; 8]; 8],
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
    pub fn valid_move(&self, row: isize, col: isize, color: Cell) -> bool {
        if self.matrix[row as usize][col as usize] != Cell::Empty {
            return false;
        }

        let opposite_cell = if color == Cell::Black { Cell::White } else { Cell::Black };
        let len = self.matrix.len() as isize;
        let mut flip = -1;

        for i in 0..8 {
            flip *= -1;

            let (mut row_add, mut col_add) = match i {
                0 | 1 => (0, 1),
                2 | 3 => (1, 0),
                4 | 5 => (1, 1),
                6 | 7 => (1, -1),
                _ => (0, 0),
            };


            row_add *= flip;
            col_add *= flip;

            let new_row = row + row_add;
            let new_col = col + col_add;

            if new_row < 0 || new_col < 0 || new_row >= len || new_col >= len {
                continue;
            }

            if self.matrix[new_row as usize][new_col as usize] != opposite_cell {
                continue;
            }

            let (mut r, mut c) = (row, col);
            loop {
                r += row_add;
                c += col_add;

                if r < 0 || c < 0 || r >= len || c >= len {
                    break;
                }

                match self.matrix[r as usize][c as usize] {
                    Cell::Empty => break,
                    cell if cell == color => return true,
                    _ => continue,
                }
            }
        }

        false
    }

    pub fn all_valid_moves(&self, color: Cell) -> Vec<(isize, isize)> {
        let mut valid_moves = Vec::new();

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                if self.valid_move(i as isize, j as isize, color) {
                    valid_moves.push((i as isize, j as isize));
                }
            }
        }

        valid_moves
    }

    pub fn application_move(&self, row: isize, col:isize, color: Cell) -> Move {
        let mut returni : Move = Move::new();
        if self.matrix[row as usize][col as usize] != Cell::Empty {
            return returni;
        }



        let opposite_cell = if color == Cell::Black { Cell::White } else { Cell::Black };
        let len = self.matrix.len() as isize;
        let mut flip = -1;

        for i in 0..8 {
            flip *= -1;

            let (mut row_add, mut col_add) = match i {
                0 | 1 => (0, 1),
                2 | 3 => (1, 0),
                4 | 5 => (1, 1),
                6 | 7 => (1, -1),
                _ => (0, 0),
            };


            row_add *= flip;
            col_add *= flip;

            let new_row = row + row_add;
            let new_col = col + col_add;

            if new_row < 0 || new_col < 0 || new_row >= len || new_col >= len {
                continue;
            }

            if self.matrix[new_row as usize][new_col as usize] != opposite_cell {
                continue;
            }

            let (mut r, mut c) = (row, col);
            let mut pos: Mova = ((row+row_add, col+col_add),(row , col) ,(row_add , col_add));
            let mut change: bool = false;
            loop {
                r += row_add;
                c += col_add;

                if r < 0 || c < 0 || r >= len || c >= len {
                    break;
                }

                match self.matrix[r as usize][c as usize] {
                    Cell::Empty => break,
                    cell if cell == color => {
                        pos.1 = (r, c);
                        change = true;
                        break;

                    },
                    _ => continue,
                }
            }
            if change {
                returni.soli.push(pos);
            }
        }

        returni
    }

    pub fn render_move(&mut self, row: isize, col: isize, color: Cell) {
        if !self.valid_move(row, col, color) {
            return;
        }

        
        let result: Move = self.application_move(row, col , color);
        self.matrix[row as usize][col as usize] = color;
        for i in result.soli {
            let (mut r, mut c) = (i.0.0, i.0.1);
            let (row_add , col_add) = i.2;
            let end_pos = i.1;
            let len = self.matrix.len() as isize;
            
            loop {
                // Check boundaries
                if r < 0 || c < 0 || r >= len || c >= len {
                    break;
                }
                
                // Check if we reached the end coordinates (the piece of our color)
                if (r, c) == end_pos {
                    break;
                }
                
                // Change the cell color
                self.matrix[r as usize][c as usize] = color;
                
                r += row_add;
                c += col_add;
            }
        }
    }
}