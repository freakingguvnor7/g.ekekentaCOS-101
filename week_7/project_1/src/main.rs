use std::io;

fn area_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_cube(side: f64) -> f64 {
    6.0 * side * side
}

fn volume_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("Choose a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    match choice {
        1 => {
            let (height, base1, base2) = read_three("height", "base1", "base2");
            println!("Area of Trapezium: {}", area_trapezium(height, base1, base2));
        }
        2 => {
            let (d1, d2) = read_two("diagonal1", "diagonal2");
            println!("Area of Rhombus: {}", area_rhombus(d1, d2));
        }
        3 => {
            let (base, altitude) = read_two("base", "altitude");
            println!("Area of Parallelogram: {}", area_parallelogram(base, altitude));
        }
        4 => {
            let side = read_one("side length");
            println!("Area of Cube: {}", area_cube(side));
        }
        5 => {
            let (radius, height) = read_two("radius", "height");
            println!("Volume of Cylinder: {}", volume_cylinder(radius, height));
        }
        _ => println!("Invalid choice"),
    }
}

fn read_one(label: &str) -> f64 {
    println!("Enter {}:", label);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

fn read_two(label1: &str, label2: &str) -> (f64, f64) {
    (read_one(label1), read_one(label2))
}

fn read_three(label1: &str, label2: &str, label3: &str) -> (f64, f64, f64) {
    (read_one(label1), read_one(label2), read_one(label3))
}