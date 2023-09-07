use std::{sync::mpsc, thread};

fn main() {
    let a = 38;
    let b = 4;
    println!("{} + {} = {}", a, b, add(a, b));
}

fn add(n1: i32, n2: i32) -> i32 {
    let mut sum = n1;
    let (count, increment) = if n2 > 0 { (n2, 1) } else { (-n2, -1) };
    let mut handles = vec![];
    let (tx, rx) = mpsc::channel();

    for _ in 0..count {
        let tx_for_thread = tx.clone();
        handles.push(thread::spawn(move || {
            tx_for_thread.send(increment).unwrap()
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for _ in 0..count {
        sum += rx.recv().unwrap();
    }

    sum
}
