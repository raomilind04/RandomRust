#[derive(Debug)]
struct Rectangle {
    length: u32, 
    width: u32,
}

#[derive(Debug)]
enum ReachMe {
    _Email(String), 
    _Phone(i32),
    Both { email: String, phone: i32 },
}

#[derive(Debug)]
struct Profile {
    name: String, 
    contact: ReachMe,
}

impl Profile {
    fn print_profile(&self) {
        println!("Profile enum: {:?}", self); 
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn fit_inside(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.width >= other.width
    }
    // Associated functions which need to be used with ::
    fn get_square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
    
fn main() {
    let r1 = Rectangle { length: 10, width: 20 }; 
    let r2 = Rectangle { length: 4, width: 10 };
    let r3 = Rectangle { length: 20, width: 10 };  

    println!("The rectagle struct r1 looks like : {:?}", r1); 
    println!("The rectagle struct r1 looks like : {:#?}", r1); 
    
    println!("The area of r1: {}", r1.area());
    
    println!("r2 can fit inside r1 ? Answer : {}", r1.fit_inside(&r2));
    println!("r3 can fit inside r1 ? Answer : {}", r1.fit_inside(&r3));

    let sq = Rectangle::get_square(10); 
    println!("sq created: {:?}", sq); 

    // Enum
    

    let person1 = Profile { 
        name: String::from("person1"), 
        contact: ReachMe::Both { 
            email: String::from("person1@email"),
            phone: 123, 
        }, 
    };

    println!("person1 : {:#?}", person1); 
    person1.print_profile(); 

    // Option
    
    plus_one(Some(5)); 
    plus_one(None); 
    plus_one(Some(100));
    if_let_else_find_ten(Some(10)); 
    if_let_else_find_ten(Some(2));
}

fn plus_one(x: Option<i32>) {
    match x {
        Some(i) => {
            println!("Result : {}", i + 1);
        }, 
        None => {
            println!("The option passed is not a number"); 
        }
        // _ => {} this is the default case for matcher
    }
}

fn if_let_else_find_ten(x: Option<i32>) {
    if let Some(10) = x {
        println!("The number is 10");
    } else {
        println!("The number passed in not 10");
    }
}

