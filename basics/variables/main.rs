fn main() {
    // Variables

    // Immutable variable
    let x = 5;
    println!("x is: {}", x);
    
    // Mutable variable
    let mut y = 10;
    y = 20;
    println!("y is now: {}", y);



    // Shadowing
    let z = 30;
    let z = z + 5;
    println!("z is: {}", z);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);
}
