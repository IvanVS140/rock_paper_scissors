#![allow(unused)]
use std::io;
use rand::Rng;

fn main() {
    let meet_vega: &str = "I am Vega. A UAC's artificial intelligence";
    let game_invite: &str = "Let's play 'ROCK, PAPER, SCISSORS' game";

    println!("\n{}. {}..", meet_vega, game_invite);

    let game_list = ["Rock", "Scissors", "Paper"];

    loop {
        let vega_choice: usize = rand::thread_rng().gen_range(0,3);
        println!("\nVega made his choice. {}\n", vega_choice + 1);

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

        if who_wins(vega_choice, user_choice) != 0 {
            break;
        }
    }
}

fn who_wins(x: usize, y: usize) -> u32 {
    if x == y {
        println!("\nDraw.");
        return 0;
    }
    else if x == (y + 2) % 3 {
        println!("Vega wins!");
        return 1;
    }
    else {
        println!("User wins!");
        return 2;
    }
}

#[test]
fn round_draw() {
    assert_eq!(who_wins(0, 0), 0);
    assert_eq!(who_wins(1, 1), 0);
    assert_eq!(who_wins(2, 2), 0);
}
#[test]
fn vega_wins() {
    assert_eq!(who_wins(0, 1), 1);
    assert_eq!(who_wins(1, 2), 1);
    assert_eq!(who_wins(2, 0), 1);
}
#[test]
fn user_wins() {
    assert_eq!(who_wins(1, 0), 2);
    assert_eq!(who_wins(2, 1), 2);
    assert_eq!(who_wins(0, 2), 2);
}