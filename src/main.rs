use beep::beep;
use std::io::prelude::*;
use std::{io::Write, thread::sleep, thread::JoinHandle, time::Duration};

mod detect;
mod display;
mod game_file;

const DEFAULT_BOARD_SIZE: usize = 8;

// https://en.wikipedia.org/wiki/Portable_Game_Notation
//

/*
🨞 🨤 🨀
*/
struct tile {
    row: u8,
    col: u8,
}

fn main() {
    let single_player_mode: bool = false;

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

    let mut new_board = display::GameBoard::ctor(None, None);

    println!("Your input was {}", input);
    let opt: u8 = match input.trim() {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        _ => 4,
    };

    // something is off about the match closure, making this neccesary,
    // I still want to keep the match tho, for when I figure it out
    if opt == 1 || opt == 2 {
        display::GameBoard::get_board(&mut new_board);
    } else if opt == 3 {
        display::GameBoard::tutorial();
    } else if opt == 4 {
        println!("Sorry, I didn't quite understand that...");
    }
}

fn splash_screen() -> JoinHandle<()> {
    let this_thread = std::thread::spawn(|| {
        let mut stdout = std::io::stdout();
        println!("   ████           █");
        println!("  █  █████████████");
        println!(" █     █████████");
        println!(" █     █  █");
        println!("  ██  █  ██        ███  ████               ████      ████");
        println!("     █  ███         ████ ████ █   ███     █ ████ █  █ ████ █");
        println!("    ██   ██          ██   ████   █ ███   ██  ████  ██  ████");
        println!("    ██   ██          ██         █   ███ ████      ████");
        println!("    ██   ██          ██        ██    ███  ███       ███");
        println!("    ██   ██          ██        ████████     ███       ███");
        println!("     ██  ██          ██        ███████        ███       ███");
        println!("      ██ █      █    ██        ██        ████  ██  ████  ██");
        println!("       ███     █     ███       ████    ██ ████ █  █ ████ █");
        println!("        ███████       ███       ███████    ████      ████");
        println!("          ███                    █████");
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

fn quit_program() {}
