use std::io::Write;


fn main() {
	let mut file = std::fs::File::create("drinks.txt").expect("create failed");

    let _comp_name = "Nigerian Brewery";
    let Cat1 = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "STAR"];
    let Cat2 = vec!["Legend", "Turbo King", "Williams"];
    let Cat3 = vec!["Maltina","Amstel Malta", "Malta Gold", "Fayrouz"];


 	file.write_all(_comp_name.as_bytes()).expect("Write to file");

 	file.write_all(b"\n\n").expect("Write to file");
 	file.write_all("Larger".as_bytes()).expect("Write to file");

 	for index in 0..Cat1.len() {
 		file.write_all(b"\n").expect("Failed to write to file");
 		file.write_all(Cat1[index].as_bytes()).expect("Failed to write to file");
 	}

 	file.write_all(b"\n").expect("Write to file");

 	file.write_all(b"\n").expect("Write to file");
 	file.write_all("Stout".as_bytes()).expect("Write to file");

 	for index in 0..Cat2.len() {
 		file.write_all(b"\n").expect("Failed to write to file");
 		file.write_all(Cat2[index].as_bytes()).expect("Failed to write to file");
 	}

 	file.write_all(b"\n").expect("Write to file");

 	file.write_all(b"\n").expect("Write to file");
 	file.write_all("Non-Alcoholic".as_bytes()).expect("Write to file");

 	for index in 0..Cat3.len() {
 		file.write_all(b"\n").expect("Failed to write to file");
 		file.write_all(Cat3[index].as_bytes()).expect("Failed to write to file");
 	}
 	println!("Data written to file");
}
    