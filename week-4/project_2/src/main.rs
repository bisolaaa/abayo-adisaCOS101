use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Experience level?(if experienced write experienced, if not, write inexperienced):");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience = input1.trim().to_lowercase();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if experience == "experienced" && age >= 40 {
        println!("Your incentive is N1,560,000");
    }
    else if experience == "experienced" && age >= 30 && age < 40 {
        println!("Your incentive is N1,480,000");
    }
    else if experience == "experienced" && age < 28 {
        println!("Your incentive is N1,300,000");
    }
    else if experience == "inexperienced" {
        println!("Your incentive is N100,000");
    }
                        
}
