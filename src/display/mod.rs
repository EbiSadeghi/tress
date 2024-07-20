use crate::DEFAULT_BOARD_SIZE;

pub struct GameBoard {
    width: usize,
    height: usize,
    board: Vec<Vec<char>>,
}

impl GameBoard {
    pub fn tutorial() {
        println!("K: ♔ (King)\nQ: ♕ (Queen)\nR: ♖ (Rook)\nB: ♗ (Bishop)\nN: ♘ (Knight)\nP: ♙ (Pawn)\nx: capture\na1-h8: movement\n\nExample: Pa2 Pa3 <press enter>");
    }

    fn initialize_board(&self) -> Vec<Vec<char>> {
        let mut array = vec![vec!['a'; self.width]; self.height];
        for ii in 0..self.height {
            for jj in 0..self.width {
                array[ii][jj] = GameBoard::wipe_tile(ii, jj);
            }
        }

        array[0][0] = '♜';
        array[1][0] = '♞';
        array[2][0] = '♝';
        array[3][0] = '♛';
        array[4][0] = '♚';
        array[5][0] = '♝';
        array[6][0] = '♞';
        array[7][0] = '♜';

        for ii in 0..8 {
            array[ii][1] = '\u{265F}';
        }

        for ii in 0..8 {
            array[ii][6] = '♙';
        }

        array[0][7] = '♖';
        array[1][7] = '♘';
        array[2][7] = '♗';
        array[3][7] = '♕';
        array[4][7] = '♔';
        array[5][7] = '♗';
        array[6][7] = '♘';
        array[7][7] = '♖';

        return array;
    }

    pub fn ctor(width: Option<usize>, height: Option<usize>) -> GameBoard {
        // todo -> implement builder design pattern or generics
        let mut new_board = GameBoard {
            width: DEFAULT_BOARD_SIZE,
            height: DEFAULT_BOARD_SIZE,
            board: vec![vec!['a'; DEFAULT_BOARD_SIZE]; DEFAULT_BOARD_SIZE],
        };

        GameBoard::initialize_board(&mut new_board);

        return new_board;
    }

    pub fn get_board(&mut self) -> &mut Vec<Vec<char>> {
        GameBoard::show_board(&self);

        return &mut self.board;
    }

    pub fn show_board(&self) {
        for yy in 0..self.height {
            print!("{} ", yy + 1);
            for xx in 0..self.width {
                print!("{} ", self.board[xx][yy]);
            }
            print!("\n");
        }
        println!("  A B C D E F G H");
    }

    pub fn wipe_tile(x: usize, y: usize) -> char {
        let tile: char;

        if (x + y) % 2 == 0 {
            tile = '□';
        } else {
            tile = '■';
        }

        return tile;
    }

    fn sample_board() {
        println!("8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜");
        println!("7 ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎");
        println!("6 □ ■ □ ■ □ ■ □ ■");
        println!("5 ■ □ ■ □ ■ □ ■ □");
        println!("4 □ ■ □ ■ □ ■ □ ■");
        println!("3 ■ □ ■ □ ■ □ ■ □");
        println!("2 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙");
        println!("1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖");
        println!("  A B C D E F G H");
        println!("🨞 🨤");
    }
}
