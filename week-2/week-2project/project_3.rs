fn main() {
	// Given data
	let principal: f64 = 510_000.0; // #510,000
	let rate: f64 = 5.0; // 5% per annum
	let time: f64 = 3; // 3 years

	// Calculate Amount
	let amount = principal * (1.0 - (rate / 100.0)).powi(time);

	// Display results
	println!("Principal (P): #{}", principal);
	println!("Rate (R): {}%", rate);
	println!("Time (T): {}years", time);
	println!("Amount (A): #{}", amount);
}