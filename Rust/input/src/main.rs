use std::io;

fn main() {
    println!("Enter your age");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let int: u32 = input.trim().parse().expect("Input is not a valid number");

    println!("You are {} old", int);
}
