fn main() {
    let x: u32 = 25;
    let f: f64 = 2.3;
    let f1: f32 = 2.4;
    let t: bool = true;
    let z: char = 'Z';
    println!("The value of x is {t}");
    let tup: (u32, f64, i8) = (400, 5.6, -3);
    let (x, y, z) = tup;
    println!("{:?}", z);
    let a: [u32; 4] = [1, 2, 3, 4];
    let r = a[2];
    println!("{:?}", a);
}
