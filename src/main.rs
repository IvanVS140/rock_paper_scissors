use std::io;
use rand::Rng;

fn main() {
    println!("\nI am Vega. A UAC's artificial Intelligence. Let's play 'ROCK, PAPER, SCISSORS' game..\n");

    loop {
    let vega_choise = rand::thread_rng().gen_range(1,4);
    println!("> Vega made his choice\n");
    
    println!("Enter your choise: 1 - ROCK, 2 - PAPER, and 3 for SCISSORS");
    let mut user_choise = String::new();
    io::stdin()
        .read_line(&mut user_choise)
        .expect("Failed to read line");

    let user_choise: u32 = match user_choise.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
        };
    if user_choise == 1 {
        println!("\n> Your choise is ROCK\n");
    }
    else if user_choise == 2 {
        println!("\n> Your choise is PAPER\n");
    }
    else {
        println!("\n> Your choise is SCISSORS\n");
    }

    if vega_choise == 1 {
        println!("> Vega chose ROCK\n");
    }  else if vega_choise == 2 {
        println!("> Vega chose PAPER\n")
    }  else {
        println!("> Vega chose SCISSORS\n")
    }

    if vega_choise == user_choise {
        println!("\nround draw\n");
        continue
        }
    else {
        if vega_choise == 1 {
            if user_choise == 3 {
                println!("Vega wins! ROCK beats SCISSORS\n");
                break
                }
            else if user_choise == 2 {
                println!("User wins! PAPPER beats ROCK\n");
                break
                }
            }
        }
        if vega_choise == 2 {
            if user_choise == 3 {
                println!("User wins! PAPPER beats ROCK\n");
                break
                }
            else if user_choise == 1 {
                println!("Vega wins! PAPPER beats ROCK\n");
                break
                }
            }
        if vega_choise == 3 {
            if user_choise == 2 {
                println!("Vega wins! SCISSORS beats PAPPER\n");
                break
                }
            else if user_choise == 1 {
                println!("User wins! ROCK beats SCISSORS\n");
                break
            }
        }
    }
}
