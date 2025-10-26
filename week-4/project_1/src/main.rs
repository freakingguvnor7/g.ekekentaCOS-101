use std::io;

fn main() {
    println!("Enter coefficients a, b, and c:");

    let a = read_input("a");
    let b = read_input("b");
    let c = read_input("c");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two real roots: x1 = {:.2}, x2 = {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: x = {:.2}", root);
    } else {
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-discriminant).sqrt() / (2.0 * a);
        println!(
            "Complex roots: x1 = {:.2} + {:.2}i, x2 = {:.2} - {:.2}i",
            real_part, imaginary_part, real_part, imaginary_part
        );
    }
}

fn read_input(name: &str) -> f64 {
    println!("Enter {}:", name);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<f64>().expect("Please enter a valid number")
}
