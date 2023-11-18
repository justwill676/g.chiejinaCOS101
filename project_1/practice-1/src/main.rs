use std::io;

fn main() {
    while let eligible_candidate_counts = 150.00{
 
    let mut _level = String::new();
    let mut _cgpa = String::new();
    let mut _class_rep = String::new();
    

    println!("Enter your level");
    io::stdin().read_line(&mut _level).expect("Failed to read input");
    let _level: f64 = _level.trim().parse().expect("Not a valid number");

    println!("Enter your cgpa");
    io::stdin().read_line(&mut _cgpa).expect("Failed to read input");
    let _cgpa: f64 = _cgpa.trim().parse().expect("Not a valid number");

    println!("Are you a class_rep\nEnter 1 for Yes or 2 for No");
    io::stdin().read_line(&mut _class_rep).expect("Failed to read input");
    let _class_rep: i32 = _class_rep.trim().parse().expect("Not a valid number");
    let mut isrep = false;
    if _class_rep == 1{
    	isrep = true;
    }else{
    	isrep = false;
    }
     
     if _cgpa >= 4.0 && _level >= 200.0 && isrep{
    {
        println!("You are eligible to vote");
    
    let mut _name = String::new();
    let mut _email = String::new();
    let mut _department = String::new();
    let mut _state_of_origin = String::new();

    println!("Enter your name");
    io::stdin().read_line(&mut _name).expect("Failed to read input");

    println!("Enter your email");
    io::stdin().read_line(&mut _email).expect("Failed to read input");

    println!("Enter your department");
    io::stdin().read_line(&mut _department).expect("Failed to read input");
    // Change the type of _department to String, if needed.

    println!("Enter your state of origin");
    io::stdin().read_line(&mut _state_of_origin).expect("Failed to read input");
}

    } else 
     {
        println!("Sorry you are not eligible to vote");
    }
}

    
}
