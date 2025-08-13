fn print_items<T: std::fmt::Display>(items: &[T]) {
    for item in items {
        println!("{}", item);
    }
}

trait Speak {
    fn speak(&self) -> String;
}
struct Human;
impl Speak for Human {
    fn speak(&self) -> String {
        "Hello, I'm human".to_string()
    }
}
struct Dog;
impl Speak for Dog {
    fn speak(&self) -> String {
        "Woof woof".to_string()
    }
}
fn main() {

    let numbers = [1, 2, 3, 4, 5];
    let words = ["hello", "world", "in", "rust"];
    
    let human: Human = Human;
    let dog = Dog;

    println!("{}", human.speak());
    println!("{}", dog.speak());
}