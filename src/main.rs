mod functions;

use std::io;
use rand::Rng;
use functions::who_wins;

fn main() {
    let meet_vega: &str = "I am Vega. A UAC's artificial intelligence";
    let game_invite: &str = "Let's play 'ROCK, SCISSORS, PAPER' game";

    println!("\n{}. {}..", meet_vega, game_invite);

    let game_list = ["Rock", "Scissors", "Paper"];

    loop {
        let vega_choice: usize = rand::thread_rng().gen_range(0,3);
        println!("\nVega made his choice.\n");

        println!("Enter your choice: 1 - ROCK, 2 - SCISSORS, and 3 for PAPER");
        let mut user_choice = String::new();
        if let Err(err) = io::stdin().read_line(&mut user_choice) {
            println!("Error occurred: {}", err);
            continue;
        }

        let user_choice = match user_choice.trim().parse::<usize>() {
            Ok(num) => num - 1,
            Err(_) => continue,
        };

        println!("\n > {} VS {} <\n", game_list[user_choice], game_list[vega_choice]);

        if who_wins(vega_choice, user_choice) != 0 {
            break;
        }
    }
}
