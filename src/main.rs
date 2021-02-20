#![allow(unused)]
use std::io;
use rand::Rng;

fn main() {
    println!("\nI am Vega. A UAC's artificial intelligence. Let's play 'ROCK, PAPER, SCISSORS' game..");
    let game_list = ["Rock", "Scissors", "Paper"];
    loop {
        let vega_choice: usize = rand::thread_rng().gen_range(0,3);
        println!("\nVega made his choice.\n");

        println!("Enter your choise: 1 - ROCK, 2 - SCISSORS, and 3 for PAPER");
        let mut user_choice = String::new();
        match io::stdin().read_line(&mut user_choice) {
            Ok(n) => {
                println!("\nLets see..")
            },
            Err(error) => {
                println!("Error occurred: {}", error);
                continue;
            },
        }

        let user_choice = match user_choice.trim().parse::<usize>() {
            Ok(num) => num - 1,
            Err(_) => continue,
        };

        println!("\n{} vs {}", game_list[user_choice], game_list[vega_choice]);

        if vega_choice == user_choice {
            println!("\nDraw.");
            continue
        }
        else if vega_choice == (user_choice + 2) % 3 {
            println!("\nVega wins! {} beats {}\n", game_list[vega_choice], game_list[user_choice]);
            break
        }
        else {
            println!("\nUser wins! {} beats {}\n", game_list[user_choice], game_list[vega_choice]);
            break
        }
    }
}
