use std::thread;
use std::time::Duration;

fn main() {
    let _j   = 0;

    thread::spawn(|| {
        for i in 1..13 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(5));
        }
    });

    thread::spawn(|| {
        for i in 1..13 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(5));
        }
    });
    for i in 1..6 {
        println!("hi number {} from the main 1st thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}