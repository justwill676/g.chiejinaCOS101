fn main() {
	
	let p = 210_000.0;
	let r = 5.0;
	let t = 3.0;

	//Compound Interest

	let x = 1.0 - (r/100.0);

	let y = f32::powf(x, t);

	let a = p * y;

	println!("The value is {}", a);

}