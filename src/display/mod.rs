use crate::DEFAULT_BOARD_SIZE;

pub struct GameBoard {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Vec<char>>,
}

impl GameBoard {
    pub fn tutorial() {
        println!("K: ♔ (King)\nQ: ♕ (Queen)\nR: ♖ (Rook)\nB: ♗ (Bishop)\nN: ♘ (Knight)\nP: ♙ (Pawn)\nx: capture\na1-h8: movement\n\nExample: Pa2 Pa3 <press enter>");
    }

    pub fn initialize_board(&mut self) -> bool {
        self.board = vec![vec!['a'; self.width]; self.height];
        for ii in 0..self.width {
            for jj in 0..self.height {
                self.board[ii][jj] = GameBoard::wipe_tile(ii, jj);
            }
        }

        self.board[0][0] = '♜';
        self.board[1][0] = '♞';
        self.board[2][0] = '♝';
        self.board[3][0] = '♛';
        self.board[4][0] = '♚';
        self.board[5][0] = '♝';
        self.board[6][0] = '♞';
        self.board[7][0] = '♜';

        for ii in 0..self.width {
            self.board[ii][1] = '\u{265F}';
        }

        for ii in 0..self.width {
            self.board[ii][6] = '♙';
        }

        self.board[0][7] = '♖';
        self.board[1][7] = '♘';
        self.board[2][7] = '♗';
        self.board[3][7] = '♕';
        self.board[4][7] = '♔';
        self.board[5][7] = '♗';
        self.board[6][7] = '♘';
        self.board[7][7] = '♖';

        true
    }

    pub fn ctor(width: Option<usize>, height: Option<usize>) -> GameBoard {
        // todo -> implement builder design pattern or generics
        let mut new_board = GameBoard {
            width: DEFAULT_BOARD_SIZE,
            height: DEFAULT_BOARD_SIZE,
            board: vec![vec!['z'; DEFAULT_BOARD_SIZE]; DEFAULT_BOARD_SIZE],
        };

        GameBoard::initialize_board(&mut new_board);

        new_board
    }

    pub fn get_board(&mut self) -> &mut Vec<Vec<char>> {
        &mut self.board
    }

    pub fn show_board(&self) {
        for yy in 0..self.height {
            print!("{} ", yy + 1);
            for xx in 0..self.width {
                print!("{} ", self.board[xx][yy]);
            }
            println!();
        }
        println!("  A B C D E F G H");
    }

    pub fn wipe_tile(x: usize, y: usize) -> char {
        let tile: char = if (x + y) % 2 == 0 { '□' } else { '■' };
        tile
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
