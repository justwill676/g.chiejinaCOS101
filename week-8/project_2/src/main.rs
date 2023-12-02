use std::io;

fn main() {
    

	 let mut username:Vec<String> = Vec::new();
	 let mut userage:Vec<String> = Vec::new();
	 let mut usernumber:Vec<String> = Vec::new();
	 let mut useraddress:Vec<String> = Vec::new();
	 let mut useryearsofexperience:Vec<String> = Vec::new();
	 let mut userplaceofwork:Vec<String> = Vec::new();
	 let mut userhle:Vec<String> = Vec::new();
	 let mut usercv:Vec<String> = Vec::new();
	 let mut userskill:Vec<String> = Vec::new();
	 let mut userproject:Vec<String> = Vec::new();

	 let mut input1 = String::new();

	 println!("[Admin only] Enter Number of candidates");
	 io::stdin().read_line(&mut input1).expect("Failed to read input");
	 let input1: i64 = input1.trim().parse().expect("Not a valid number");

   for _val in 0..input1 {


		let mut _name = String::new();
		let mut _age = String::new();
		let mut phone_number = String::new();
		let mut _address = String::new();
		let mut years_of_experience = String::new();
		let mut place_of_work = String::new();
		let mut _hle = String::new();
		let mut _cv = String::new();
		let mut _skill = String::new();
		let mut latest_project = String::new();

		println!("Input your fullname");
		io::stdin().read_line(&mut _name).expect("Failed to read input");
		let _name: String = _name.trim().parse().expect("Not a valid number");
		//pushing array into element
		username.push(_name.to_string());

		println!("Enter your age");
		io::stdin().read_line(&mut _age).expect("Failed to read input");
		let _age: i64 = _age.trim().parse().expect("Not a valid number");
		//pushing array into an element
		userage.push(_age.to_string());

		println!("Enter your phone number");
		io::stdin().read_line(&mut phone_number).expect("Failed to read input");
		let phone_number: i64 = phone_number.trim().parse().expect("Not a valid number");
		//pushing array into an element	
		usernumber.push(phone_number.to_string());

		println!("Enter your home/work Adress");
		io::stdin().read_line(&mut _address).expect("Failed to read input");
		let _address: String = _address.trim().parse().expect("Not a valid number");
		//pushing array into an element
		useraddress.push(_address.to_string());

		println!("Enter your years of experience ");
		io::stdin().read_line(&mut years_of_experience).expect("Failed to read input");
		let years_of_experience: i64 = years_of_experience.trim().parse().expect("Not a valid number");
		//pushing array into an element 
		useryearsofexperience.push(years_of_experience.to_string());

		println!("Enter your place of work");
		io::stdin().read_line(&mut place_of_work).expect("Failed to read input");
		let place_of_work: String = place_of_work.trim().parse().expect("Not a valid number");
		//pushing array into an element
		userplaceofwork.push(place_of_work.to_string());

		println!("Input your highest level of education attained");
		io::stdin().read_line(&mut _hle).expect("Failed to read input");
		let _hler: String = _hle.trim().parse().expect("Not a valid number");
		//pushing array into an element
		userhle.push(_hle.to_string());

		println!("Enter your CV");
		io::stdin().read_line(&mut _cv).expect("Failed to read input");
		let _cv: String = _cv.trim().parse().expect("Not a valid number");
		//pushing array into an element
		usercv.push(_cv.to_string());

		println!("Enter any special skill you have. If any:");
		io::stdin().read_line(&mut _skill).expect("Failed to read input");
		let _skill: String = _skill.trim().parse().expect("Not a valid number");
		//pushing array into an element
		userskill.push(_skill.to_string());

		println!("Upload your lastest project");
		io::stdin().read_line(&mut latest_project).expect("Failed to read input");
		let latest_project: String = latest_project.trim().parse().expect("Not a valid number");
		//pushing array into an element
		userproject.push(latest_project.to_string());

		println!("Your details have been uploaded successfully");

    }
     if let Some((index, _max)) = useryearsofexperience.iter().enumerate().max_by_key(|(_, &ref value)| value) {
        
        println!("The Candidate with the Highest Number of Experience is...");
        println!("Candiate's name: {:?}",username[index]);
        println!("Candiate's age: {:?}",userage[index]);
        println!("Candiate's number: {:?}",usernumber[index]);
        println!("Candiate's address: {:?}",useraddress[index]);
        println!("Candiate's years of experience: {:?}",useryearsofexperience[index]);
        println!("Candidate's place of work: {:?}", userplaceofwork[index]);
		println!("Candidate's highest education level: {:?}", userhle[index]);
		println!("Candidate's specialized skills: {:?}", userskill[index]);
		println!("Candidate's latest project: {:?}", userproject[index]);
    }
    else {
        println!("empty vector");
    }


}
