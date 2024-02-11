extern crate rand;

use std::io;
use std::collections::HashMap;
use std::cmp::Ordering;
use rand::Rng;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Level {
    Easy,
    Medium,
    Hard,
}

lazy_static! {
    static ref LEVEL_TO_ATTEMPTS_MAPPING: HashMap<Level, i32> = {
        let mut map = HashMap::new();
        map.insert(Level::Easy, 10);
        map.insert(Level::Medium, 5);
        map.insert(Level::Hard, 3);
        map
    };
}

fn main() {
    println!("Welcome to the game");
    for _i in 1..50 {
        print!("-");
    }
    println!();

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut _max_attempts = 0;

    loop {
        println!("Choose a difficulty level: Easy / Medium / Hard");
        let mut level_input = String::new();
        io::stdin().read_line(&mut level_input).expect("Failed to read line");

        let chosen_level = match parse_level(&level_input.trim()) {
            Some(level) => level,
            None => {
                println!("Invalid level chosen. Please choose a valid level.");
                continue;
            }
        };

        match LEVEL_TO_ATTEMPTS_MAPPING.get(&chosen_level) {
            Some(&attempts) => {
                _max_attempts = attempts;
                break;
            }
            None => {
                println!("Difficulty level not found. Please choose a valid level.");
            }
        }
    }
    for _i in 1..50 {
        print!("-");
    }
    println!(); 

    println!("You have {} attempts.", _max_attempts);

    for _i in 1..50 {
        print!("-");
    }
    println!(); 

    let mut attempts_made = 1; 
    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        if guess.trim().to_uppercase() == "QUIT" {
            println!("Quitting the game");
            break;
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guess {}, is not a valid number. Try again!", guess.trim());
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess: {} is too small!", guess),
            Ordering::Greater => println!("Your guess: {} is too big!", guess),
            Ordering::Equal => {
                println!("Correct! You win");
                break;
            }
        }

        attempts_made += 1; 

        if attempts_made > _max_attempts {
            println!("Fail! No more attempts left"); 
            break; 
        }

        let remaining_attempts = _max_attempts - attempts_made;
        println!("Remaining attempts: {}", remaining_attempts);

        for _i in 1..50 {
            print!("-");
        }
        println!(); 
    }
}

fn parse_level(level: &str) -> Option<Level> {
    match level.trim().to_lowercase().as_str() {
        "easy" => Some(Level::Easy),
        "medium" => Some(Level::Medium),
        "hard" => Some(Level::Hard),
        _ => None
    }
}
