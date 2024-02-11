use std::thread; 
use std::time::Duration; 
use std::collections::HashMap; 

struct Cacher<T> where T: Fn(i32) -> i32 {
    calculation: T, 
    value: HashMap<i32, i32>, 
}

impl<T> Cacher<T> where T: Fn(i32) -> i32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation, 
            value: HashMap::<i32, i32>::new(), 
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        if self.value.contains_key(&arg) {
            println!("something: {}", self.value.get(&arg).unwrap()); 
            *(self.value).get(&arg).unwrap()
        } else {
            let v = (self.calculation)(arg);
            self.value.insert(arg, v); 
            v
        }
    }
}

fn main() {
    let mut exp_res = Cacher::new(|num| {
        println!("calculating something"); 
        // simulate exp operation
        thread::sleep(Duration::from_secs(2)); 
        num 
    }); 
    
    println!("{}", exp_res.value(2)); 
    println!("{}", exp_res.value(10)); 
    println!("{}", exp_res.value(2)); 
    println!("{}", exp_res.value(2)); 
    println!("{}", exp_res.value(2));
    println!("{}", exp_res.value(10)); 
    println!("{}", exp_res.value(2)); 
    println!("{}", exp_res.value(10)); 
    println!("{}", exp_res.value(10)); 
    println!("{}", exp_res.value(10)); 
}




