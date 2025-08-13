fn print_items<T: std::fmt::Display>(items: &[T]) {
    for item in items {
        println!("{}", item);
    }
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let words = ["hello", "world", "in", "rust"];

    print_items(&numbers);
    print_items(&words);
}
