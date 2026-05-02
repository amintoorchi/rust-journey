use std::io;

fn main() {

    println!("Hello, Mr Alireza Boroujerdian");

    println!("Please Enter a number:");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input value");

    let num: i32 = user_input.trim().parse().expect("Not a valid number");
    let doubled = num * 2;

    println!("Double of {} is: {}", num, doubled);
}