use std::{fs, i64}; 
use std::error::Error; 

#[derive(Debug)]
pub struct Config {
    filename: String, 
    phrase: String, 
}

impl Config {
    pub fn new(fname: Option<String>, query: Option<String>) -> Option<Config> {
        match (fname, query) {
            (Some(filename), Some(phrase)) => {
                Some(Config {
                    filename, 
                    phrase,
                })
            }, 
            (None, None) => None, 
            (_, _) => {
                panic!("System error"); 
            }
        } 
    }
}

pub fn run(config: Config)  -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename.as_str())?; 
    let result = search_content(config.phrase.as_str(), content.as_str());
    println!("Found {} matches", result.len());
    for (line, line_count) in result {
        println!("Line number: {}", line_count); 
        println!("Preview: {}", line); 
    } 
    Ok(())
}

pub fn search_content<'a>(query: &'a str, content: &'a str) -> Vec<(&'a str, i64)>{
    let mut matched_results = Vec::new();
    let mut line_count: i64 = 1; 
    for line in content.lines() {
        if line.contains(query) {
            matched_results.push((line, line_count)); 
        }
        line_count += 1; 
    }
    matched_results 
}


