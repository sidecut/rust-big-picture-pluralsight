use std::thread;

fn main() {
    let a = 38;
    let b = 4;
    println!("{} + {} = {}", a, b, add(a, b));
}

fn add(n1: i32, n2: i32) -> i32 {
    let mut sum = n1;
    let (count, increment) = if n2 > 0 { (n2, 1) } else { (-n2, -1) };
    let mut handles = vec![];

    for _ in 0..count {
        handles.push(thread::spawn(|| {
            sum += increment;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    sum
}
