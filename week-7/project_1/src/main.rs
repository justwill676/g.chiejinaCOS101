use std::io;

fn main() {

	let mut base1 = String::new();
    let mut base2 = String::new();
    let mut area1 = String::new();
    let mut height1 = String::new();
    let mut diagonal1 = String::new();
    let mut diagonal2 = String::new();
    let mut area2 = String::new();
    let mut base3 = String::new();
    let mut altitude = String::new();
    let mut area3 = String::new();
    let mut side_length = String::new();
    let mut area4 = String::new();
    let mut radius = String::new();
    let mut height2 = String::new();
    let mut volume = String::new();



    println!("Select equation");
    println!("1 = Area of a Trapezium");
    println!("2 = Area of a Rhombus");
    println!("3 = Area of a Parallelogram");
    println!("4 = Area of a cube");
 	println!("5 = Volume of a cylinder");

 	println!("Choose equation");
 	io::stdin().read_line(&mut input).expect("Failed to read input");
    let select: f64 = input.trim().parse().expect("Not a valid number");


	//Functions to calculate the area of a trapezium
	if select == 1.00 {
	println!("Enter lenght of the first base");
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let base1: f64 = base1.trim().parse().expect("Not a valid number");

	println!("Enter lenght of the second base");
    io::stdin().read_line(&mut base2).expect("Failed to read input");
    let base2: f64 = base2.trim().parse().expect("Not a valid number");

    println!("Enter the height of the trapezium");
    io::stdin().read_line(&mut height1).expect("Failed to read input");
	let height1: f64 = height1.trim().parse().expect("Not a valid number");

	 let area1: f64 = 0.5 * (base1 + base2) * height1;

    println!("The area of the trapezium is: {}", area1);
}
// Function to calculate the area of a rhombus
	else if select == 2.00 {
	println!("Enter lenght of the first diagonal");
    io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
    let diagonal1: f64 = diagonal1.trim().parse().expect("Not a valid number");

	println!("Enter lenght of the second diagonal");
    io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
    let diagonal2: f64 = diagonal2.trim().parse().expect("Not a valid number");

    let area2: f64 = 0.5 * diagonal1 * diagonal2;

    println!("The area of the rhombus is: {}", area2);
}
// Function to calculate the area of a parallelogram
	else if select == 3.00 {
	println!("Enter lenght of the base");
    io::stdin().read_line(&mut base3).expect("Failed to read input");
    let base3: f64 = base3.trim().parse().expect("Not a valid number");

 
	println!("Enter lenght of the altitude");
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude: f64 = altitude.trim().parse().expect("Not a valid number");

    let area3: f64 = base3 * altitude;

    println!("The area of the parallelogram is: {}", area3);
}
// Function to calculate the area of a cube
	else if select == 4.00 {
	println!("Enter lenght of side");
    io::stdin().read_line(&mut side_length).expect("Failed to read input");
    let side_length: f64 = side_length.trim().parse().expect("Not a valid number");

    let area4: f64 = 6.0 * side_length * side_length;

    println!("The area of the cube is: {}", area4);
}
// Function to calculate the volume of a cylinder
	else if select == 5.00{
	println!("Enter radius of the cylinder");
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius: f64 = radius.trim().parse().expect("Not a valid number"); 

    println!("Enter radius of the cylinder");
    io::stdin().read_line(&mut height2).expect("Failed to read input");
    let height2: f64 = height2.trim().parse().expect("Not a valid number");

    let volume: f64 = 3.14 * radius * radius * height2;

    println!("The volume of the cylinder is: {}", volume);
 }
}
