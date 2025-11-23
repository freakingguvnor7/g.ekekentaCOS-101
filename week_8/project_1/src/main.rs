use std::io;

fn main() {
    println!("Welcome to the APS Level Checker for the Federal Government of Nigeria!");

    // Prompt for profession
    println!("Enter your profession (public_servant, office_admin, academic, lawyer, teacher):");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).unwrap();
    let profession = profession.trim().to_lowercase();

    // Prompt for APS level
    println!("Enter your APS level (e.g., 2, 5, 9, 11, 13):");
    let mut aps_input = String::new();
    io::stdin().read_line(&mut aps_input).unwrap();
    let aps_level: u8 = match aps_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid APS level.");
            return;
        }
    };

    // Determine role
    let role = match profession.as_str() {
        "public_servant" => match aps_level {
            1..=2 => "Intern",
            3..=5 => "Administrator",
            6..=8 => "Senior Administrator",
            9..=10 => "Office Manager",
            11..=13 => "Director",
            _ => "CEO",
        },
        "office_admin" => match aps_level {
            1..=2 => "Intern",
            3..=5 => "Administrator",
            6..=8 => "Senior Administrator",
            9..=10 => "Office Manager",
            11..=13 => "Director",
            _ => "CEO",
        },
        "academic" => match aps_level {
            3..=5 => "Research Assistant",
            6..=8 => "PhD Candidate",
            9..=10 => "Post-Doc Researcher",
            11..=13 => "Senior Lecturer",
            _ => "Dean",
        },
        "lawyer" => match aps_level {
            1..=2 => "Paralegal",
            3..=5 => "Junior Associate",
            6..=8 => "Associate",
            9..=10 => "Senior Associate 1-2",
            11..=13 => "Senior Associate 3-4",
            _ => "Partner",
        },
        "teacher" => match aps_level {
            1..=2 => "Placement",
            3..=5 => "Classroom Teacher",
            6..=8 => "Snr Teacher",
            9..=10 => "Leading Teacher",
            11..=13 => "Deputy Principal",
            _ => "Principal",
        },
        _ => "Unknown profession",
    };

    println!("Your role is: {}", role);
}