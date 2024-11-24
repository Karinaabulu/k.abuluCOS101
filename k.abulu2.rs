use std::io;

fn main (){
    //staff members name
    let mut name = String::new();
    println!("Please enter your name:");
    io::stdin().read_line(&mut name).expect("Fail to read line");
    let name = name.trim();

    let mut input1 = String::new();
    println!("Please enter the number of papers you have published:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let A: u64 = input1.trim().parse().expect("Not a valid number");

    if A >= 3 && A <= 5{
        let incentive1 = 500_000;
        println!("Your Incentive is: N{}", incentive1);
    }

    else if A >= 5 && A < 10{
        let incentive2 = 800_000;
        println!("Your Incentive is: N{}", incentive2);
    }

    else if  >= 10{
        let incentive3 = 1_000_000;
        println!("Your Incentive is: N{}", incentive3);
    }

    else if A >= 3 && A <= 5{
        let incentive4 = 100_000
        println!("Your Incentive is: N{}", incentive4);
    }

    

    println!("Staff Member: {}", name);
    println!("You have published {}paper",a);
    println!("Your Incentive is N{}",incentive);
}
