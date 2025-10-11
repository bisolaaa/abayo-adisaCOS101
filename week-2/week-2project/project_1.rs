fn main() {
	// Given data
	let principal:f64 = 520_000_000.0; // #520,000,000
	let rate:f64 = 10.0; // 10% per annum
	let years:i32 = 5; // 5 years

	// Calculate amount
	let amount = principal * (1.0 + (rate/100.0)).powi(years);

	// Calculate compound interest
	let compound_interest = amount - principal;

	// Display results
	println!("Principal (P): #{}", principal);
	println!("Rate (R): {}%", rate);
	println!("Time (n): {} years", years);
	println!("Total Amount (A): #{}", amount);
	println!("Compound Interest (CI): #{}", compound_interest);


}