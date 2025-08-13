use std::io;

// Define the greetings function
fn greetings(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    println!("Please enter your name:");

    // Create a variable to store the input
    let mut name = String::new();

    // Read the user input
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Trim the newline character from the input
    let name = name.trim();

    // Call the greetings function with the user's name
    greetings(name);
}

