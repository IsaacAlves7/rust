fn main() {
    let s = String::from("Hello");
    takes_ownership(&s);
    // s is no longer valid here
    println!("{s}");

}

fn takes_ownership(s1: &String){
    println!("{s1}");
}
