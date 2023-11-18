use std::io;

struct Patient {
    name: String,
    date_of_birth: f64,
    email_address: String,
    phone_number: f64,
    no_of_siblings: f64,
    no_of_children: f64,
    medical_diagnosis: String,
    village_of_residence: String,
}

fn main() {
    const MAX_PATIENTS: usize = 100;
    let mut patients_counts = 0;

    let mut patients: Vec<Patient> = Vec::new();

    while patients_counts < MAX_PATIENTS {
        let mut patient = Patient {
            name: String::new(),
            date_of_birth: 0.0,
            email_address: String::new(),
            phone_number: 0.0,
            no_of_siblings: 0.0,
            no_of_children: 0.0,
            medical_diagnosis: String::new(),
            village_of_residence: String::new(),
        };

        println!("Enter patient information:");
        io::stdin().read_line(&mut patient.name).expect("Failed to read input");

        // ... (similarly read other patient information)

        // Check eligibility and calculate discount
        let discount = match patient.medical_diagnosis.as_str() {
            "Alzheimer" if patient.date_of_birth > 50.0 && patient.no_of_children > 4.0 && patient.village_of_residence == "Akpabom" => 0.20,
            "Arrhythmia" if patient.date_of_birth == 30.0 && patient.no_of_siblings > 4.0 && patient.village_of_residence == "Ngbauji" => 0.05,
            "Chronic Kidney Disease" if patient.date_of_birth > 40.0 && patient.no_of_children > 3.0 && patient.no_of_siblings > 3.0 && patient.village_of_residence == "Atabrikang" => 0.15,
            "Diabetes" if patient.date_of_birth > 28.0 && patient.date_of_birth < 45.0 && patient.no_of_children >= 2.0 && patient.no_of_children <= 4.0 && patient.village_of_residence == "Okorobilom" => 0.10,
            "Arthritis" if patient.date_of_birth > 58.0 && patient.no_of_children > 5.0 && patient.no_of_siblings > 5.0 && patient.village_of_residence == "Emeremen" => 0.10,
            _ => 0.0,
        };

        // Calculate discounted amount
        let discounted_amount = match patient.medical_diagnosis.as_str() {
            "Alzheimer" | "Arrhythmia" | "Chronic Kidney Disease" | "Diabetes" | "Arthritis" => {
                let amount = match patient.medical_diagnosis.as_str() {
                    "Alzheimer" => 1_200_000.00,
                    "Arrhythmia" => 550_000.00,
                    "Chronic Kidney Disease" => 1_500_000.00,
                    "Diabetes" => 800_000.00,
                    "Arthritis" => 450_000.00,
                    _ => 0.0,
                };
                amount - (discount * amount)
            }
            _ => 0.0,
        };

        // Display patient information and discounted amount
        println!("Patient Information:");
        println!("Name: {}", patient.name);
        // ... (display other patient information)
        println!("Medical Diagnosis: {}", patient.medical_diagnosis);
        println!("Discounted Amount: N{:.2}", discounted_amount);

        // Increment patient count
        patients_counts += 1;

        // Add patient to the vector
        patients.push(patient);
    }
}
