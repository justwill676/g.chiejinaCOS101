fn main() {
	
	let p = 520_000_000.0;
	let r = 10.0;
	let t = 5.0;

	//Compound Interest

	let x = 1.0 + (r/100.0);

	let y = f32::powf(x, t);

	let a = p * y;

	println!("Projected Amount is {}", a);

	let ci = a - p;

	println!("Compounded Interest is {}", ci);
}