use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the number for Z");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1: i64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the nth term");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let input2: i64 = input2.trim().parse().expect("Not a valid number");

    let y = input2;

    for i in 1..=y {
    	println!("{} x {} = {}", input1, i, input1 * i );
    }


}
