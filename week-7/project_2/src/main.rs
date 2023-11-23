use std::io;

fn main() {
    let mut no_of_siblings = String::new();
    let mut input1 = String::new();
    let mut _age = String::new();
    let mut _married = String::new();
    let mut _single = String::new();
    let mut _student = String::new();
    let mut _worker = String::new();
    let mut _university = String::new();
    let mut course_of_study = String::new();
    let mut _offsprings = String::new();
    let mut _city = String::new();
    let mut _waec = String::new();
    let mut name_of_secondary_school = String::new();
    let mut class_level = String::new(); 

    // empty array 
     let mut myarr:Vec<String> = Vec::new();

    println!("Enter the number of siblings");
    io::stdin().read_line(&mut no_of_siblings).expect("Failed to read input");
    let no_of_siblings: i64 = no_of_siblings.trim().parse().expect("Not a valid number");

    for i in 0..no_of_siblings{

     println!("Enter your first name");
     io::stdin().read_line(&mut input1).expect("Failed to read input");
     //pushing array into element
     myarr.push(input1.to_string());

     println!("Enter your age");
     io::stdin().read_line(&mut _age).expect("Failed to read input");
     let _age: f64 = _age.trim().parse().expect("Not a valid number");
     //pushing array into element
     myarr.push(_age.to_string());

        if _age > 18.00 {
         println!("Are you married?\nEnter 1 for Yes or 2 for No");
         io::stdin().read_line(&mut _married).expect("Failed to read input");
         let _married: f64 = _married.trim().parse().expect("Not a valid number");
         //pushing array into element
         myarr.push(_married.to_string());

         println!("Are you single?\nEnter 1 for Yes or 2 for No");
         io::stdin().read_line(&mut _single).expect("Failed to read input");
         let _single: f64 = _single.trim().parse().expect("Not a valid number");
         //pushing array into element
         myarr.push(_single.to_string());

           if _single == 1.00 {
             println!("Are you a student?\nEnter 1 for Yes or 2 for No");
             io::stdin().read_line(&mut _student).expect("Failed to read input");
             let _student: f64 = _student.trim().parse().expect("Not a valid number");
             //pushing array into element
             myarr.push(_student.to_string());

             println!("Are you a worker?\nEnter 1 for Yes or 2 for No");
             io::stdin().read_line(&mut _worker).expect("Failed to read input");
             let _worker: f64 = _worker.trim().parse().expect("Not a valid number");
             //pushing array into element
             myarr.push(_worker.to_string());


              if _married == 1.00 {
                 println!("Do you have children?\nEnter 1 for Yes or 2 for No");
                 io::stdin().read_line(&mut _offsprings).expect("Failed to read input");
                 let _offsprings: f64 = _offsprings.trim().parse().expect("Not a valid number");
                 //pushing array into element
                 myarr.push(_offsprings.to_string());

                 println!("What city does your family live in?");
                 io::stdin().read_line(&mut _city).expect("Failed to read input");
                 //pushing array into element
                 myarr.push(_city.to_string());
                }
            }
        }

       else if _age < 18.00 {
         println!("Have you written WAEC?\nEnter 1 for Yes or 2 for No");
         io::stdin().read_line(&mut _waec).expect("Failed to read input");
         let _waec: f64 = _waec.trim().parse().expect("Not a valid number"); 
         //pushing array into element
         myarr.push(_waec.to_string());

            if _waec == 1.00 {
             println!("Enter the name of your secondary school?");
             io::stdin().read_line(&mut name_of_secondary_school).expect("Failed to read input");
             //pushing array into element
             myarr.push(name_of_secondary_school.to_string());

               if _waec == 2.00 {
                 println!("Enter your current class level");
                 io::stdin().read_line(&mut class_level).expect("Failed to read input");
                 //pushing array into element
                  myarr.push(class_level.to_string());

                }
         
	        }
  		}
  		for val in myarr.iter() {
  			println!("{:?}",val );
  		}
    }
}


