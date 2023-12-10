use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Convicted Ministers.txt").expect("create failed");

    let file_name = "Convicted Ministers";
    let _number: Vec<i32> = vec![1,2,3,4,5];
    let name_of_commisioners: Vec<&str> = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let _ministry: Vec<&str> = vec!["Internal Affairs", "Justice", "Defence", "Power & Steel", "Petroleum"];
    let Geopolitical_zone: Vec<&str> = vec!["South West", "North East", "South South", "South West", "South East"];

 	// empty array
 	let mut ministers: Vec<(i32, &str, &str, &str)> = Vec::new();

    for i in 0.._number.len(){
    	let ministers_info = (_number[i],name_of_commisioners[i],_ministry[i],Geopolitical_zone[i]);
    	ministers.push(ministers_info);
    }

    let header = ["Number","Name Of Commisioners", "Ministry", "Geopolitical Zone"];

    file.write_all(b"\t\t").expect("Failed to write to file");
 	file.write_all(file_name.as_bytes()).expect("Failed to write to file");
 	file.write_all(b"\n\n").expect("Failed to write to file");

 	for index in &header {
 		file.write_all(index.as_bytes()).expect("Failed to write to file");
 		file.write_all(b"\t").expect("Failed to write to file");	
 	}

 	for index in &ministers {
 		file.write_all(b"\n").expect("Failed to write to file");
 		file.write_all(index.0.to_string().as_bytes()).expect("Failed to write to file");
	 	file.write_all(b"\t").expect("Failed to write to file");

 		file.write_all(index.1.as_bytes()).expect("Failed to write to file");
	 	file.write_all(b"\t").expect("Failed to write to file");

 		file.write_all(index.2.as_bytes()).expect("Failed to write to file");
	 	file.write_all(b"\t\t").expect("Failed to write to file");

	    file.write_all(index.3.as_bytes()).expect("Failed to write to file");
	 	file.write_all(b"\t").expect("Failed to write to file");
 	}
}
