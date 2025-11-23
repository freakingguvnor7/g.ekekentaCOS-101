use std::io;

// Define a struct for developers
struct Developer {
    name: String,
    experience: u32,
}

fn main() {
    let mut developers: Vec<Developer> = Vec::new();

    println!("Enter number of candidates:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: usize = input.trim().parse().expect("Please enter a valid number");

    for i in 1..=num {
        println!("Enter name of candidate {}:", i);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Enter years of experience for {}:", name.trim());
        let mut exp = String::new();
        io::stdin().read_line(&mut exp).expect("Failed to read line");
        let experience: u32 = exp.trim().parse().expect("Please enter a valid number");

        developers.push(Developer {
            name: name.trim().to_string(),
            experience,
        });
    }

    // Find the most experienced developer
    let mut top_dev = &developers[0];
    for dev in &developers {
        if dev.experience > top_dev.experience {
            top_dev = dev;
        }
    }

    println!(
        "The most experienced developer is {} with {} years of experience.",
        top_dev.name, top_dev.experience
    );
}