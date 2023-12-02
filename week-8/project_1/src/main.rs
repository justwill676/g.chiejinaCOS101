use std::io;

fn main() {
    let mut input1 = String::new();
    let mut years_of_experience = String::new();
    let mut staff_position = String::new();
    let mut office_administrator = String::new();
    let mut _academic = String::new();
    let mut _lawyer = String::new();
    let mut _teacher = String::new();

// empty array 
 let mut myarr1:Vec<String> = Vec::new();
 let mut myarr2:Vec<String> = Vec::new();
 let mut myarr3:Vec<String> = Vec::new();
 let mut myarr4:Vec<String> = Vec::new();
 let mut myarr5:Vec<String> = Vec::new();
 let mut myarr6:Vec<String> = Vec::new();


 println!("Enter your first name");
 io::stdin().read_line(&mut input1).expect("Failed to read input");
 //pushing array into element
 

 println!("Enter years of work experience");
 io::stdin().read_line(&mut years_of_experience).expect("Failed to read input");
 let years_of_experience: f64 = years_of_experience.trim().parse().expect("Not a valid number");
 //pushing array into element

    println!("Select your type of lawyer");
    println!("1 = Paralegal ");
    println!("2 = Junior associate");
    println!("3 = Associate");
    println!("4 = Senior Associate 1-2");
 	println!("5 = Senior Associate 3-4");
 	println!("6 = Partner");

   	println!("Choose your type of lawyer");
 	io::stdin().read_line(&mut _lawyer).expect("Failed to read input");
    let _select: f64 = _lawyer.trim().parse().expect("Not a valid number");

 

	if years_of_experience <= 2.00 && _select == 1.00 {
		println!("Enter your office administration");
		io::stdin().read_line(&mut office_administrator).expect("Failed to read input");
		//pushing array into element
		myarr1.push(office_administrator.to_string());

	    println!("Enter your academic");
		io::stdin().read_line(&mut _academic).expect("Failed to read input");
		//pushing array into element
		myarr1.push(_academic.to_string());

	    println!("Enter the type of teacher you are");
	    io::stdin().read_line(&mut _teacher).expect("Failed to read input");
		//pushing array into element
		myarr1.push(_teacher.to_string());

        println!("You are qualified for APS 1-2\n CONGRATULATIONS!!");
        
	}	
    else {
      println!("ERROR!\nYou are not qualified for APS 1-2. Try again");
	}

    if years_of_experience >= 3.00 && years_of_experience <= 5.00 && _select == 2.00 {
		println!("Enter your office administration");
		io::stdin().read_line(&mut office_administrator).expect("Failed to read input");
		//pushing array into element
		myarr2.push(office_administrator.to_string());

		println!("Enter your academic");
		io::stdin().read_line(&mut _academic).expect("Failed to read input");
		//pushing array into element
		myarr2.push(_academic.to_string());

		println!("Enter the type of teacher you are");
		io::stdin().read_line(&mut _teacher).expect("Failed to read input");
		//pushing array into element
		myarr2.push(_teacher.to_string());
		println!("You are qualified for APS 3-5\n CONGRATULATIONS!!"); 
	}
	else{
	 println!("You are not qualified for APS 3-5. Try again");
	}



    if years_of_experience >= 6.00 && years_of_experience <= 8.00 && _select == 3.00 {
		println!("Enter your office administration");
	    io::stdin().read_line(&mut office_administrator).expect("Failed to read input");
		//pushing array into element
		myarr3.push(office_administrator.to_string());

	    println!("Enter your academic");
		io::stdin().read_line(&mut _academic).expect("Failed to read input");
	    //pushing array into element
		myarr3.push(_academic.to_string());

		println!("Enter the type of teacher you are");
		io::stdin().read_line(&mut _teacher).expect("Failed to read input");
		//pushing array into element
		myarr3.push(_teacher.to_string());

		println!("You are qualified for APS 5-8\n CONGRATULATIONS!!");
	}
	else {
     println!("You are not qualified for APS 5-8. Try again");
    }


	if years_of_experience >= 9.00 && years_of_experience <= 10.00 && _select == 4.00 {
	 println!("Enter your office administration");
	 io::stdin().read_line(&mut office_administrator).expect("Failed to read input");
	 //pushing array into element
	 myarr4.push(office_administrator.to_string());

	 println!("Enter your academic");
	 io::stdin().read_line(&mut _academic).expect("Failed to read input");
     //pushing array into element
	 myarr4.push(_academic.to_string());

	 println!("Enter the type of teacher you are");
     io::stdin().read_line(&mut _teacher).expect("Failed to read input");
	 //pushing array into element
	 myarr4.push(_teacher.to_string());

	 println!("You are qualified for EL1\n CONGRATULATIONS!!");
    }
    else {
     println!("You are not qualified for EL1. Try again");
	}



	if years_of_experience >= 11.00 && years_of_experience <= 13.00 && _select == 5.00 {
     println!("Enter your office administration");
     io::stdin().read_line(&mut office_administrator).expect("Failed to read input");
     //pushing array into element
     myarr5.push(office_administrator.to_string());

 	 println!("Enter your academic");
     io::stdin().read_line(&mut _academic).expect("Failed to read input");
 	 //pushing array into element
	 myarr5.push(_academic.to_string());

 	 println!("Enter the type of teacher you are");
 	 io::stdin().read_line(&mut _teacher).expect("Failed to read input");
 	 //pushing array into element
	 myarr5.push(_teacher.to_string());

	 println!("You are qualified for EL2\n CONGRATULATIONS!!");
	}
	else{
	 println!("You are not qualified for EL2. Try again");
    }


    if years_of_experience > 13.00 && _select == 6.00 {
     println!("Enter your office administration");
     io::stdin().read_line(&mut office_administrator).expect("Failed to read input");
     //pushing array into element
     myarr6.push(office_administrator.to_string());

 	 println!("Enter your academic");
     io::stdin().read_line(&mut _academic).expect("Failed to read input");
 	 //pushing array into element
	 myarr6.push(_academic.to_string());

 	 println!("Enter the type of teacher you are");
 	 io::stdin().read_line(&mut _teacher).expect("Failed to read input");
 	 //pushing array into element
	 myarr6.push(_teacher.to_string());

	 println!("You are qualified for SES\n CONGRATULATIONS!!");
	} 
	else {
		println!("You are not qualified for SES. Try again");
	}
			
   for val in myarr1.iter() {
		println!("{:?}",val );

		for val in myarr2.iter() {
		 println!("{:?}",val );

		    for val in myarr3.iter() {
			 println!("{:?}",val );

			    for val in myarr4.iter(){                  
			     println!("{:?}",val );

			        for val in myarr5.iter() {
					 println!("{:?}",val );

					    for val in myarr6.iter() {
                         println!("{:?}",val );
	                    }
	                }
			    }  
	        }
	    }
    }
}



