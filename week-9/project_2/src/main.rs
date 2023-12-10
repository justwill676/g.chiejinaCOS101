use std::io::Write;


fn main() {
	let mut file = std::fs::File::create("Pausmis.txt").expect("create failed");

    let _comp_name = "PAU SMIS";
    let student_name: Vec<&str> = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_num: Vec<&str> = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department: Vec<&str> = vec!["Accounting", "Economics", "Computer Science", "Electrical Enigineering", "Mechanical Enigineering"];
    let level: Vec<i32> = vec![300,100,200,200,100];	

    let mut candidates: Vec<(&str, &str, &str, i32)> = Vec::new();

    for i in 0..student_name.len(){
    	let candidates_info = (student_name[i],matric_num[i],department[i],level[i]);
    	candidates.push(candidates_info);
    }

    let header = ["Student Name", "Matric Number", "Department", "Level"];

    file.write_all(b"\t\t").expect("Failed to write to file");
 	file.write_all(_comp_name.as_bytes()).expect("Failed to write to file");
 	file.write_all(b"\n\n").expect("Failed to write to file");

 	for index in &header {
 		file.write_all(index.as_bytes()).expect("Failed to write to file");
 		file.write_all(b"\t").expect("Failed to write to file");	
 	}
    

 	for index in &candidates {
 		file.write_all(index.0.as_bytes()).expect("Failed to write to file");
	 	file.write_all(b"\t\t").expect("Failed to write to file");

 		file.write_all(index.1.as_bytes()).expect("Failed to write to file");
	 	file.write_all(b"\t\t").expect("Failed to write to file");

	    file.write_all(index.2.as_bytes()).expect("Failed to write to file");
	 	file.write_all(b"\t\t").expect("Failed to write to file");

	 	file.write_all(index.3.as_bytes()).expect("Failed to write to file");
	 	file.write_all(b"\t\t").expect("Failed to write to file");


 	}
}
