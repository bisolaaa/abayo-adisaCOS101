// Rust program to find the roots of a quadratic equation
use std::io;
fn main() {
    println!("Find the roots of a quadratic equation: ax² + bx + c = 0");
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("There are two distinct roots:");
        println!("x1 = {}", root1);
        println!("x2 = {}", root2);
    }
    else if discriminant == 0.0 {
        let root = -b / 2.0 * a;
        println!("There is only one real root:");
        println!("x = {}", root);
    }
    else if discriminant < 0.0 {
        let real_part = -b / 2.0 * a;
        let imaginary_part = (-discriminant).sqrt() / 2.0 * a;
        println!("There are no real roots: ");
        println!("x1 = {} + {}i", real_part, imaginary_part);
        println!("x2 = {} - {}i", real_part, imaginary_part);
 }
}
