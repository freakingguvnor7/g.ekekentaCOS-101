use std::io;

fn main() {
    println!("Enter employee's experience status (yes/no):");
    let experienced = read_experience();

    println!("Enter employee's age:");
    let age = read_input();

    let incentive = calculate_incentive(experienced, age);
    println!("Annual incentive: ₦{:.2}", incentive);
}

fn read_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<u32>().expect("Please enter a valid number")
}

fn read_experience() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.trim().to_lowercase().as_str() {
        "yes" => true,
        "no" => false,
        _ => {
            println!("Invalid input. Assuming 'no'.");
            false
        }
    }
}

fn calculate_incentive(experienced: bool, age: u32) -> f64 {
    if experienced {
        if age >= 40 {
            1_560_000.0
        } else if age >= 30 {
            1_480_000.0
        } else if age < 28 {
            1_300_000.0 * 12.0 // Monthly incentive converted to annual
        } else {
            // Experienced but age between 28 and 29
            1_300_000.0 * 12.0
        }
    } else {
        100_000.0
    }
}