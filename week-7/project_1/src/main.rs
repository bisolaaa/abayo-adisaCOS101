use std::io;

fn area_trapezium(b1: f64, b2: f64, h:f64) -> f64 {
    0.5 * h * (b1 + b2)
}
fn area_rhombus(d1: f64, d2: f64) -> f64 {
    0.5 * d1 * d2
}
fn area_parallelogram(a: f64, b: f64) -> f64 {
    a * b
}
fn area_cube(s: f64) -> f64 {
    6.0 * s * s
}
fn volume_cylinder(r: f64, h:f64) -> f64 {
    std::f64::consts::PI * r * r * h
}

fn read_input(prompt: &str) -> f64 {
    loop {
        println!("{}",prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,

            Err(_) => {
                println!("Enter a valid input");
                continue;
            }
        }
    }
}
    fn main() {
        loop {
        println!("\nPick a calculation:
                    1. Area of trapezium
                    2. Area of rhombus
                    3. Area of parallelogram
                    4. Area of cube
                    5. Volume of cylinder");

        let choice:f64 = read_input("Enter the number of the formula you want to calculate:");

        match choice {
            1.0 => {
                let b1:f64 = read_input("Enter base 1:");
                let b2:f64 = read_input("Enter base 2:");
                 let h:f64 = read_input("Enter height:");
                 let result = area_trapezium(b1, b2, h);
                 println!("Area of trapezium: {}\n", result);
            }

            2.0 => {
                let d1:f64 = read_input("Enter diagonal 1:");
                let d2:f64 = read_input("Enter diagonal 2:");
                 let result = area_rhombus(d1, d2,);
                 println!("Area of rhombus: {}\n", result);
            }

            3.0 => {
                let a:f64 = read_input("Enter altitude:");
                let b:f64 = read_input("Enter base:");
                 let result = area_parallelogram(a, b);
                 println!("Area of parallelogram: {}\n", result);
            }

            4.0 => {
                let s:f64 = read_input("Enter side length:");
                 let result = area_cube(s);
                 println!("Area of cube: {}\n", result);
            }

            5.0 => {
                let r:f64 = read_input("Enter radius:");
                let h:f64 = read_input("Enter height:");
                 let result = volume_cylinder(r, h);
                 println!("Volume of cylinder: {}\n", result);
            } 

            _ => {
                println!("You didn't enter a valid input. Please try again");
                continue;
            }
        }

        println!("Do you want to make another calculation? yes/no");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Not a valid string");
        let choice:String = choice.trim().to_lowercase();

        if choice == "no" {
            println!("Thank you for using my calculator!");
            break;
        }
    }
    }
