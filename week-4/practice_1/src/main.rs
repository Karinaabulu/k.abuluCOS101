// Rust program to output name and age
 
use std::io;

fn main() {
    println!("\nStudent Information Managment System!");

    // input name
    println!("\nPlease enter ur name.");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Fail to read input");
    println!("Your name is: {}",name);

    // inpute age
    println!("\nEnter Your age.");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Fail to read input");
    let age = 32 age.trim().parse().expect("Input not any integer");
    println!("Your age is:{}"age);
}
