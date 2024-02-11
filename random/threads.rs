use std::thread; 
use std::sync::mpsc; 
use std::sync::{Mutex, Arc}; 
use std::time::Duration; 

// fn main() {
//     let (tx, rx) = mpsc::channel(); 
//     let tx1 = tx.clone(); 
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("thread 1: 1"), 
//             String::from("thread 1: 2"),
//             String::from("thread 1: 3"),
//             String::from("thread 1: 4"),
//         ]; 
//         for val in vals {
//             tx1.send(val).unwrap(); 
//             thread::sleep(Duration::from_secs(1)); 
//         }
//     }); 
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("thread 2: 1"),
//             String::from("thread 2: 2"),
//             String::from("thread 2: 3"),
//             String::from("thread 2: 4"),
//         ]; 
//         for val in vals {
//             tx.send(val).unwrap(); 
//             thread::sleep(Duration::from_secs(2)); 
//         }
//     }); 
//     
//     for received in rx {
//         println!("Main: {}", received);
//     }
// }
//

fn main() {
    let counter = Arc::new(Mutex::new(0)); 
    let mut handles = vec![]; 
    for _ in 1..10 {
        let counter = counter.clone(); 
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); 
            *num += 1; 
        }); 
        handles.push(handle); 
    }
    for handle in handles {
        handle.join().unwrap(); 
    }
    println!("Result {}", *counter.lock().unwrap());
}
