use std::io;

fn main() {
    println!("Welcome to the Food Ordering System!");
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3200");
    println!("F = Fried Rice & Chicken      - N3000");
    println!("A = Amala & Ewedu Soup        - N2500");
    println!("E = Eba & Egusi Soup          - N2000");
    println!("W = White Rice & Stew         - N2500");

    let mut total = 0.0;

    loop {
        println!("\nEnter food code (P, F, A, E, W) or Q to quit:");
        let mut code = String::new();
        io::stdin().read_line(&mut code).unwrap();
        let code = code.trim().to_uppercase();

        if code == "Q" {
            break;
        }

        let price = match code.as_str() {
            "P" => 3200.0,
            "F" => 3000.0,
            "A" => 2500.0,
            "E" => 2000.0,
            "W" => 2500.0,
            _ => {
                println!("Invalid code. Try again.");
                continue;
            }
        };

        println!("Enter quantity:");
        let mut qty = String::new();
        io::stdin().read_line(&mut qty).unwrap();

        let qty: f64 = match qty.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid quantity. Try again.");
                continue;
            }
        };

        total += price * qty;
        println!("Added N{:.2} to total.", price * qty);
    }

    println!("\nTotal before discount: N{:.2}", total);

    if total > 10000.0 {
        let discount = total * 0.05;
        total -= discount;
        println!("Discount of 5% applied: N{:.2}", discount);
    }

    println!("Final total: N{:.2}", total);
    println!("Thank you!");
}