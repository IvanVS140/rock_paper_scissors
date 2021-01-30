use std::io;
use rand::Rng;

fn main() {
    println!("\nI am Vega. A UAC's artificial intelligence. Let's play 'ROCK, PAPER, SCISSORS' game..");
    let game_list = ["", "Rock", "Paper", "Scissors"];
    loop {
        let vega_choice = rand::thread_rng().gen_range(1,4);
        println!("\nVega made his choice.\n");

        println!("Enter your choise: 1 - ROCK, 2 - PAPER, and 3 for SCISSORS");
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        let user_choice: u32 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("\nVega chose {}", game_list[vega_choice as usize]); // for now
        println!("User chose is {}", game_list[user_choice as usize]);

        if vega_choice == user_choice {
            println!("\nDraw.");
            continue
        }
        else if vega_choice == 1 && user_choice == 3 || vega_choice == 2 && user_choice == 1 || vega_choice == 3 && user_choice == 2 {
            println!("\nVega wins! {} beats {}\n", game_list[vega_choice as usize], game_list[user_choice as usize]);
            break
        }
        else {
            println!("\nUser wins! {} beats {}\n", game_list[user_choice as usize], game_list[vega_choice as usize]);
            break
        }
    }
}
