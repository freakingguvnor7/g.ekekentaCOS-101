struct Laptop {
    brand: String,
    unit_price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.unit_price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        unit_price: 650_000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        unit_price: 755_000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        unit_price: 550_000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        unit_price: 850_000,
    };

    let quantity = 3;

    println!(
        "{}: {} units → ₦{}",
        hp.brand,
        quantity,
        hp.total_cost(quantity)
    );
    println!(
        "{}: {} units → ₦{}",
        ibm.brand,
        quantity,
        ibm.total_cost(quantity)
    );
    println!(
        "{}: {} units → ₦{}",
        toshiba.brand,
        quantity,
        toshiba.total_cost(quantity)
    );
    println!(
        "{}: {} units → ₦{}",
        dell.brand,
        quantity,
        dell.total_cost(quantity)
    );

    let total = hp.total_cost(quantity)
        + ibm.total_cost(quantity)
        + toshiba.total_cost(quantity)
        + dell.total_cost(quantity);

    println!("Total cost for 3 laptops from each brand is: ₦{}", total);
}