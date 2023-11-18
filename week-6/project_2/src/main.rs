use std::io;

fn main() {
    let mut rpis_count = 500.00;

    while rpis_count > 0.00 {
        println!("Number of papers published with their incentive");
        let mut incentive = 0.0;

        println!("Enter your name");
        let mut name_of_researcher = String::new();
        io::stdin().read_line(&mut name_of_researcher).expect("Failed to read input");

        println!("Enter number of papers published");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let number_of_papers_published: f64 = input.trim().parse().expect("Not a valid number");

        if number_of_papers_published >= 10.0 {
            incentive = 1_000_000.00;
        } else if number_of_papers_published >= 5.0 {
            incentive = 800_000.00;
        } else if number_of_papers_published >= 3.0 {
            incentive = 500_000.00;
        } else if number_of_papers_published >= 2.0 {
            incentive = 100_000.00;
        }

        println!("Researcher: {}, Papers Published: {}, Incentive: {}", name_of_researcher, number_of_papers_published, incentive);

        // Update the rpis_count or add any other logic as needed
        rpis_count += 1.0;
    }
}
