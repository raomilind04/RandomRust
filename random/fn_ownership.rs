fn main() {
    let s = String::from("Hello");
    let _s1 = s.clone();
    let len = get_len(&s[..]); 
    println!("s : {} with len : {}", s, len);
    let s2 = String::from("what is going on");
    let first_word = get_first_word(&s2[..]); 
    println!("first word is {}", first_word); 
}


fn get_len(s: &str) -> usize {
    s.len()
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); 
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..] 
}
