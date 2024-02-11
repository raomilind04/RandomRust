use std::fs::File; 
use std::io::ErrorKind; 
use std::io; 
use std::io::Read; 

fn main() {
    let file = match File::open("test.txt") {
        Ok(file) => file, 
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("The file does not exist. Creating a new file");
            match File::create("test.txt") {
                Ok(created_file) => created_file, 
                Err(e) => {
                    panic!("File creation failed with error: {:#?}", e); 
                }, 
            }
        },
        Err(error) => {
            panic!("File could not be opened. Here is why: {:#?}", error); 
        }, 
    }; 
 
    println!("Contents of the file are : {}", 
        get_file_content(&String::from("test2.txt")[..]).unwrap());

    println!("Contents of the file are : {}", 
        get_file_content(&String::from("txt2.txt")[..]).expect("File is missing"));

}

fn get_file_content(s: &str) -> Result<String, io::Error> {
    let mut buff = String::new(); 
    File::open(s)?.read_to_string(&mut buff)?;  
    Ok(buff)
}

