use std::io;
fn main() {
    loop {
    let mut input1 = String::new();
    let mut input2 = String::new();
    

    println!("Welcome to my restraunt!");
    println!("Select your desired item from our menu:
        P = Pounded yam/Edinkaiko soup- N3,200
        F = Fried rice & Chicken- N3,000
        A = Amala & Ewedu soup- N2,500
        E = Eba & Egusi soup- N2,000
        W = White rice & strew- N2,500");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let food:String = input1.trim().to_lowercase();

    println!("How much of your item do you want?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let qty:f32 = input2.trim().parse().expect("Not a valid number");

    let p:f32 = 3_200.0;
    let f:f32 = 3_000.0;
    let a:f32 = 2_500.0;
    let e:f32 = 2_000.0;
    let w:f32 = 2_500.0;

    let total_p = 3_200.0 * qty;
    let total_f = 3_000.0 * qty;
    let total_a = 2_500.0 * qty;
    let total_e = 2_000.0 * qty;
    let total_w = 2_500.0 * qty;

    let last_pricep = 0.95 * total_p;
    let last_pricef = 0.95 * total_f;
    let last_pricea = 0.95 * total_a;
    let last_pricee = 0.95 * total_e;
    let last_pricew = 0.95 * total_w;


    if food == "p" && total_p > 10_000.0 {
        println!("Your total was: N{} 
        But you have recieved a 5% discount!
        So, your final price is: N{} ", total_p, last_pricep);
    } else if food == "p" && total_p <= 10_000.0 {
        println!("Your total is: {}", total_p);
    }


    if food == "f" && total_f > 10_000.0 {
        println!("Your total was: N{} 
        But you have recieved a 5% discount!
        So, your final price is: N{} ", total_f, last_pricef);
    } else if food == "f" && total_f <= 10_000.0{
        println!("Your total is: {}", total_f);
    }


    if food == "a" && total_a > 10_000.0 {
        println!("Your total was: N{} 
        But you have recieved a 5% discount!
        So, your final price is: N{} ", total_a, last_pricea);
    } else if food == "a" && total_a <= 10_000.0{
        println!("Your total is: {}", total_a);
    }


    if food == "e" && total_e > 10_000.0 {
        println!("Your total was: N{} 
        But you have recieved a 5% discount!
        So, your final price is: N{} ", total_e, last_pricee);
    } else if food == "e" && total_e <= 10_000.0{
        println!("Your total is: {}", total_e);
    }


    if food == "w" && total_w > 10_000.0 {
        println!("Your total was: N{} 
        But you have recieved a 5% discount!
        So, your final price is: N{} ", total_w, last_pricew);
    } else if food == "w" && total_w <= 10_000.0{
        println!("Your total is: {}", total_w);
    }

    let mut choice = String::new();

    println!("Next customer? (yes/no)");
    io::stdin().read_line(&mut choice).expect("Not a valid string");
    let choice = choice.trim().to_lowercase();

    if choice == "no" {
        println!("Thank you so much for patronizing us!");
        break;
    }
}
}
