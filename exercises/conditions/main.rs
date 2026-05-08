// quera Practice question

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let t: i32 = input.trim().parse().expect("Invalid number");

    if t > 100 {
        println!("Steam");
    } else if t < 0 {
        println!("Ice");
    } else {
        println!("Water");
    }
}