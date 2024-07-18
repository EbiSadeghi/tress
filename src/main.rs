use beep::beep;
use std::io::prelude::*;
use std::{io::Write, thread::sleep, thread::JoinHandle, time::Duration};

mod detect;
mod display;
mod game_file;

// https://en.wikipedia.org/wiki/Portable_Game_Notation
//

/*
♜♞♝♛♚♝♞♜
♟︎♟︎♟︎♟︎♟︎♟︎♟︎♟︎

♙♙♙♙♙♙♙♙
♖♘♗♕♔♗♘♖

🨞 🨤 🨀
*/

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

    println!("Your input was {}", input);
    match input.trim() {
        "1" => display::board(),
        "2" => display::board(),
        "3" => display::tutorial(),
        _ => println!("Sorry, I didn't quite understand that..."),
    }
}

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

fn quit_program() {}
