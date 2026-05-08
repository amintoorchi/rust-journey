use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let mut iter = input.trim().split(" ");

    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();

    if a + b + c == 180 && a > 0 && b > 0 && c > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}