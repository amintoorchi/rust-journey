fn main(){
    'loop_one: loop {
        println!("Loop => 1");
        loop {
            println!("Loop => 2");
            break 'loop_one;
        }
    }
}