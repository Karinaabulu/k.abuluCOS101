use std::io;

fn main (){
	let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Are you a Class Rep?: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("What level are you in?: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let level:i32 = input2.trim().parse().expect("Not a valid number");
    

    println!("What is your cgpa?: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let cgpa:f32 = input3.trim().parse().expect("Not a valid number");


    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    
   
    println!("Enter your name: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");

    println!("Enter your Email: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");

    println!("Enter your Department: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");

    println!("Enter your State of Origin: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");


     if cgpa >= 4.0 && level > 100{
      println!("You can Vote!");
    } else{
    	println!("Sorry, you are not eligible to vote");
    }   


}


// Question 2 for CA1


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


