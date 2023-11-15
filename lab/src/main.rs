
use std::io;

fn main() {
    println!("Hello, world!");

    let a = 255u8;
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");

    let b: u8 = b.trim().parse().expect("Invalid input");

    //let c = a+b;// panic
    let c = a.wrapping_add(b);
    
    println!("a+b = {c}");
}
