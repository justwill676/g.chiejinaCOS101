fn main() {
     
     let fullname = "Chibudum John Umeh";
     let department = "Computer Science";
     let uni = "Pan-Atlantic University";


    
     let mut school = "School of Science".to_string();
     // push string 
     school.push_str(" and Technology");
     // check lenght
     println!("The lenght my fullname is: {}",fullname.len());
     println!("I am a student of {} Department", department);
     println!("{}",school);
     println!("{}",uni);
}
