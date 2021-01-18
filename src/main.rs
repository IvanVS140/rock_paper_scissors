use std::io;
use rand::Rng;
//use std::cmp::Ordering;

fn main() {
    println!("I am Vega. A UAC's artificial Intelligence. Let's play game..");

    let vega_choise = rand::thread_rng().gen_range(1,4);
    println!("vega_choise is: {}", vega_choise);
    if vega_choise == 1 {
        println!("rock");
    }  else if vega_choise == 2 {
        println!("papper")
    }  else {
        println!("scissirs")
    }
    loop {
    println!("Enter your choise: 1 - ROCK, 2 - PAPPER, and 3 for SCISSORS");
    let mut user_choise = String::new();
    io::stdin()
        .read_line(&mut user_choise)
        .expect("Failed to read line");

    let user_choise: u32 = match user_choise.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
        };
    println!("its {}", user_choise);
    if vega_choise == user_choise {
        println!("round draw");
        continue
        }
    else {
        if vega_choise == 1 {
            if user_choise == 3 {
                println!("Vega wins! ROCK > SCISSORS");
                break
                }
        }
        if vega_choise == 1 {
            if user_choise == 2 {
                println!("User wins! ROCK > SCISSORS");
                break
                }
            }   
        }
    }
}
