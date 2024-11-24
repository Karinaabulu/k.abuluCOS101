use std::io;

fn main (){

    let mut name = String::new();
    println!("Please enter your name:");
    io::stdin().read_line(&mut name).expect("Not a Valid String");
    let name = name.trim();

    let mut input1 = String::new();
    println!("Please enter the number of papers you have published:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p: u64 = input1.trim().parse().expect("Not a valid number 'p'");

    let incentive = if p>= 3 && p<=5{
        500_00
    } else if p > 5 && p< 10{
        800_00
    } else if p>= 10{
        1_000_000
    }else {
        100_000 // Default case for p <3
    };
    println!("Staff Member: {}", name);
    println!("You have published {}paper",p);
    println!("Your Incentive is N{}",incentive);
}

