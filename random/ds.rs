use std::collections::HashMap; 

fn main() {
    let mut v: Vec<i32> = Vec::new(); 
    v.push(10); 
    v.push(20); 
    for (index, &element) in v.iter().enumerate() {
        println!("element {} is {}", index+1, &element);
    }
    
    let v2 = vec![10, 20, 30]; 
    for (index, &element) in v2.iter().enumerate() {
        println!("element {} is {}", index+1, &element);
    }

    let first = &v2[0]; 
    println!("The first element is {}", first);

    let s1 = "string1".to_string(); 
    let s2 = String::from("string2");
    println!("s1 : {}, s2 : {}", s1, s2);

    let mut s3 = String::from("Hello"); 
    s3.push_str(" World");
    println!("s3 : {}", s3);

    let movestr = String::from("move"); 
    let notmove = String::from(" | not moved");
    println!("merged : {}", movestr + &notmove);
    println!("merged via format: {}", format!("{}-{}", s1, s2));

    let p = "Hello world world hello"; 
    for c in p.chars() {
        print!("{}", c);
    }
    println!("");
    for b in p.bytes() {
        print!("{} ", b);
    }
    println!(""); 
    
    let mut m = HashMap::new(); 
    m.insert(String::from("Hello"), 10); 
    m.insert(String::from("World"), 20); 

    for (key, value) in &m {
        println!("{}-{}", key, value);
    }

    let mut m2 = HashMap::new(); 
    for word in p.split_whitespace() {
        let count = m2.entry(word).or_insert(0); 
        *count += 1; 
    }
    println!("map m2 : {:#?}", m2);
}
