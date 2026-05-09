fn main() {
    let mut number = 0;

    while number <= 100 {
        if number % 2 != 0 {
            println!("Number {}, is not Zog", number);
        } else {
            println!("Number {} => Zog", number);
        }
        number += 1;
    }
}