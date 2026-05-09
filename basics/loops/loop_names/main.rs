// fn main(){
//     'loop_one: loop {
//         println!("Loop => 1");
//         loop {
//             println!("Loop => 2");
//             break 'loop_one;
//         }
//     }
// }




fn main(){
    'loop_one: loop {
        println!("Loop => 1");
        loop {
            println!("Loop => 2");
            loop {
                println!("Loop => 3");
                loop {
                    println!("Loop => 4");
                    loop {
                        println!("Loop => 5");
                        break 'loop_one;
                    }
                }
            }
        }
    }
}