#[derive(Debug)]
struct Student{
    name: String,
    grade: char,
    age: u32,
}
//tuple structs
struct Color(i32, i32, i32);
struct Marker;

impl Student{
    fn intoduce(&self){
        println!("My name is {} and I am {} years old", self.name, self.age);
    }
}


fn main() {
    let student1 = Student {
        name: String::from("Alice"),
        grade: 'A',
        age: 20,
    };
    let student2 = Student{
        name: String::from("John"),
        ..student1
    };

    let black = Color(0, 0, 0);

    println!("Student2 is {:?}", student2);

    
}
