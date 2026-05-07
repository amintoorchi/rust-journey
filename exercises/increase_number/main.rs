use std::io;

fn main() {
    println!("Enter a number to increase:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("feiled to read line");

    let number: i32 = input.trim().parse().expect("Failed to parse integer");

    println!("Increased number is : {}", number * 2);
}