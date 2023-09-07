mod accumulator;

use std::sync::{Arc, Mutex};
use std::thread;

use accumulator::Accumulator;

fn main() {
    let a = 38;
    let b = 4;
    println!("{} + {} = {}", a, b, add(a, b));
}

fn add(n1: i32, n2: i32) -> i32 {
    let acc = Arc::new(Mutex::new(Accumulator::new(n1)));
    let (count, increment) = if n2 > 0 { (n2, 1) } else { (-n2, -1) };
    let mut handles = vec![];

    for _ in 0..count {
        let guarded_acc = Arc::clone(&acc);
        handles.push(thread::spawn(move || {
            let mut guarded_acc = guarded_acc.lock().unwrap();
            guarded_acc.add(increment);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_acc = acc.lock().unwrap();
    final_acc.get_sum()
}
