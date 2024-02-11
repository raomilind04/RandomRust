use std::{io, process};
use mini_grep::{Config, run}; 

#[derive(Debug)]
enum ProcessingResult {
    OPTIONS, 
    SEARCH,
    INVALID,
}

#[derive(Debug)]
enum Menu {
    QUIT, 
    HELP,
    INVALID,
}

#[derive(Debug)]
struct ParsingResponse {
    result: ProcessingResult, 
    config: Option<Config>,
    menu: Option<Menu>, 
}


fn main() {
    println!("Mini grep"); 
    println!("q/quit: Exit"); 
    println!("help: open help menu"); 
    println!("Input format: filename phrase"); 
    print_line(); 
    loop {
        let input = match read_input() {
            Ok(s) => s,
            Err(e) => {
                println!("Failure : {}. Try Again !", e.to_string()); 
                continue; 
            },
        };
        let parsing_response: ParsingResponse = parse_input(&input[..]);
        match parsing_response.result {
            ProcessingResult::OPTIONS => {
                match parsing_response.menu {
                    Some(op) => {
                        handle_menu_operation(op); 
                    }, 
                    None => {
                        println!("Unable to identify operation. Try help for more information");
                    }
                }
            }, 
            ProcessingResult::SEARCH => {
                if let Err(e) = run(parsing_response.config.unwrap()) {
                    println!("Unable to process request: {}", e);
                }
            }, 
            ProcessingResult::INVALID => {
                println!("The input entered is incorrect. Type \"help\" to understand more.");
            }
        }
        print_line(); 
    }    
}

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input)?; 
    Ok(input.trim().to_string()) 
}

fn parse_input(s: &str) -> ParsingResponse{
    let parsed_inputs: Vec<&str> = s.split_whitespace().collect(); 
    match parsed_inputs.len() {
        0 => {
            ParsingResponse {
                result: ProcessingResult::INVALID, 
                config: None, 
                menu: None, 
            }
        }, 
        1 => {
            ParsingResponse {
                result: ProcessingResult::OPTIONS, 
                config: Config::new(None, None),  
                menu: Some(match parsed_inputs[0].to_lowercase().as_str() {
                    "help" => Menu::HELP, 
                    "quit" => Menu::QUIT, 
                    "q" => Menu::QUIT,
                    _ => Menu::INVALID, 
                }), 
            }
        },
        2 => {
            ParsingResponse {
                result: ProcessingResult::SEARCH,
                config: Config::new(Some(parsed_inputs[0].to_string()), 
                    Some(parsed_inputs[1].to_string())),
                menu: None,
            }
        }, 
        _ => {
            ParsingResponse {
                result: ProcessingResult::INVALID, 
                config: Config::new(None, None), 
                menu: None, 
            }
        },
    }
}

fn print_line() {
    for _i in 1..50 {
        print!("-"); 
    }
    println!(); 
}

fn handle_menu_operation(op: Menu) {
    match op {
        Menu::HELP => {
             println!("q/quit: Exit"); 
             println!("help: open help menu"); 
            println!("Input format: filename phrase"); 
        }, 
        Menu::QUIT => {
            println!("Quitting the program...");
            print_line(); 
            process::exit(1);
        }, 
        Menu::INVALID => {
           println!("Invalid menu option. Try help for more information");
        }
    } 
}


