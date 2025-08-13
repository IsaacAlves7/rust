use core::num;

#[derive(Debug)]
enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
//pattern matching
fn process_message(msg: Message){
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move {mut x, y } => {
            x = x+1;
            println!("Moving to x: {}, y: {}", x, y)
        },
        Message::Write(text) => println!("Text message is: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to: R{}, G{}, B{}", r, g, b),
    }
}
//prelude
fn safe_divide(numerator: i32, denominator: i32) -> Option<f32> {
    if denominator == 0 {
        None 
    }
    else {
        Some(numerator as f32 / denominator as f32)
    }
}

fn main() {
    let result = safe_divide(10, 2);
    println!("{:?}", result);

    let result2 = safe_divide(10, 0);
    println!("{:?}", result2);
}
