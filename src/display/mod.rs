pub fn tutorial() {
    println!("K: ♔ (King)\nQ: ♕ (Queen)\nR: ♖ (Rook)\nB: ♗ (Bishop)\nN: ♘ (Knight)\nP: ♙ (Pawn)\nx: capture\na1-h8: movement\n\nExample: Pa2 Pa3 <press enter>");
}

fn initialize_board(width: usize, height: usize) -> Vec<Vec<char>> {
    let mut array = vec![vec!['a'; width]; height];
    for ii in 0..8 {
        for jj in 0..8 {
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

// pub fn batch(alpha:f32, features:&[[f32; 1]; 4], labels:&[f32]) -> Vec<f32>
pub fn show_board(width: usize, height: usize, array: &Vec<Vec<char>>) {
    for yy in 0..height {
        print!("{} ", yy + 1);
        for xx in 0..width {
            print!("{} ", array[xx][yy]);
        }
        print!("\n");
    }
    println!("  A B C D E F G H");
}

pub struct GameBoard {
    width: usize,
    height: usize,
    board: Vec<Vec<char>>,
}

impl GameBoard {
    pub fn get_board() {
        let width: usize = 8;
        let height: usize = 8;
        let board = initialize_board(width, height);

        show_board(width, height, &board);
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
}
