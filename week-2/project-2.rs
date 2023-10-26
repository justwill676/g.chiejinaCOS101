fn main() {
	
	println!();

	// defining laptops

	let t = 2.0 * 450_000.00;
	let m = 1.0 * 1_500_000.00;
	let h = 3.0 * 750_000.00;
	let d = 3.0 * 2_850_000.00;
	let a = 1.0 * 250_000.00;

	// Sum

	let sum = t + m + h + d + a;
	println!("The total sum is {}",sum);

	// Average
	let n = 10.0;
	let avg = sum / n;
	println!("The average of the sales record is {}",avg);
}