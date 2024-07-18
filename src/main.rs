use beep::beep;
use std::io::prelude::*;
use std::{io::Write, thread::sleep, thread::JoinHandle, time::Duration};

// https://en.wikipedia.org/wiki/Portable_Game_Notation
//
/*
♜♞♝♛♚♝♞♜
♟︎♟︎♟︎♟︎♟︎♟︎

♙♙♙♙♙♙♙♙
♖♘♗♕♔♗♘♖

🨞 🨤 🨀



*/
fn main() {
    // Spin up threads
    let loading_thread = splash_screen();

    // Join the threads
    loading_thread.join().unwrap();

    //let musicThread = music();
    //musicThread.join().unwrap();

    println!("\nChoose an option:");
    println!("New Game...      (enter \"1\")");
    println!("Continue Game... (enter \"2\")");
    println!("Tutorial...      (enter \"3\")\n");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Your input was {}", input);
    match input.trim() {
        "1" => display_board(),
        "2" => display_board(),
        "3" => display_tutorial(),
        _ => println!("Sorry, I didn't quite understand that..."),
    }
}

/*
pub fn read_lines() -> Vec<String> {
    let mut vec = Vec::new();
    let mut string = String::new();

    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();

    while let Ok(len) = stdin_lock.read_line(&mut string) {
        if len > 0 {
            vec.push(string);
            string = String::new();
        } else {
            break;
        }
    }

    vec
}
*/

fn display_tutorial() {
    println!("K: ♔ (King)\nQ: ♕ (Queen)\nR: ♖ (Rook)\nB: ♗ (Bishop)\nN: ♘ (Knight)\nP: ♙ (Pawn)\nx: capture\na1-h8: movement\n\nExample: Pa2 Pa3 <press enter>");
}

fn display_board() {
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

fn check_collision() {}

fn check_castle() {}

fn check_game_bound() {}

fn check_en_passant() {}

fn check_convert_pawn() {}

fn check_valid_move() {}

fn check_p_move() {}

fn check_r_move() {}

fn check_b_move() {}

fn check_k_move() {}

fn check_q_move() {}

fn check_pinned_piece() {}

fn check_check() {}

fn check_checkmate() {}

fn log_move_to_cfg() {}

fn splash_screen() -> JoinHandle<()> {
    let this_thread = std::thread::spawn(|| {
        let mut stdout = std::io::stdout();
        println!("   ████           █");
        println!("  █  █████████████                                          ");
        println!(" █     █████████                                            ");
        println!(" █     █  █                                                 ");
        println!("  ██  █  ██        ███  ████               ████      ████   ");
        println!("     █  ███         ████ ████ █   ███     █ ████ █  █ ████ █");
        println!("    ██   ██          ██   ████   █ ███   ██  ████  ██  ████ ");
        println!("    ██   ██          ██         █   ███ ████      ████      ");
        println!("    ██   ██          ██        ██    ███  ███       ███     ");
        println!("    ██   ██          ██        ████████     ███       ███   ");
        println!("     ██  ██          ██        ███████        ███       ███ ");
        println!("      ██ █      █    ██        ██        ████  ██  ████  ██ ");
        println!("       ███     █     ███       ████    ██ ████ █  █ ████ █  ");
        println!("        ███████       ███       ███████    ████      ████   ");
        println!("          ███                    █████                      ");
        println!("");

        for ii in 0..100 {
            print!("\rEbi Sadeghi\t\t\t\t\tLoading {}%...", ii);
            stdout.flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        println!();
    });

    return this_thread;
}

fn music() -> JoinHandle<()> {
    let this_thread = std::thread::spawn(|| {
        beep(440);
        std::thread::sleep(std::time::Duration::from_millis(500));
        beep(880);
        std::thread::sleep(std::time::Duration::from_millis(500));
        beep(0);
    });

    return this_thread;
}

fn game_file_playback() {}

fn game_file_continue() {}

fn game_file_new() {}

fn game_file_quit() {}

fn quit_program() {}

fn set_player_mode() {}

fn wipe_tile() {}
