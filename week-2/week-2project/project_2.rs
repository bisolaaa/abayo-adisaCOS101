fn main() {
	// Given data
	let t:f64 = 450_000.0;
	let m:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let d:f64 = 2_850_000.0;
	let a:f64 = 250_000.0;

	// Calculate Sum
	let sum = t + m + hp + d + a;

	// Calculate Average
	let total_quantity:f64 = 10.0;
	let average = sum / total_quantity;

	// Display results 
	println!("t: {}", t);
	println!("m: {}", m);
	println!("hp: {}", hp);
	println!("d: {}", d);
	println!("a: {}", a);
	println!("Sum (S): {}", sum);
	println!("Total quantity (T): {}", total_quantity);
	println!("Average (A): {}", average);
}