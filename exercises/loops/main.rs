use std::io;
fn main(){

    loop {

        println!("Select onec of numbers");
        println!("");
        println!("1) Start Core");
        println!("2) Stop Core");
        println!("3) Chack Status");
        println!("4) Support");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: i32 = input.trim().parse().unwrap();


        if input == 1 || input == 2 || input == 3  || input == 4  {
            println!("Your select => {}", input);
            break;
        }else {
            println!("Invalid input");
        }

    }


}