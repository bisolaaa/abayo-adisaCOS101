struct Laptop {
    brand: String,
    unit_price: u32,
    quantity: u32,
}

impl Laptop {
    fn total_cost(&self) -> u32 {
        self.unit_price * self.quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        unit_price: 650_000,
        quantity: 3,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        unit_price: 755_000,
        quantity: 3,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        unit_price: 550_000,
        quantity: 3,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        unit_price: 850_000,
        quantity: 3,
    };

    let total = hp.total_cost() + ibm.total_cost() + toshiba.total_cost() + dell.total_cost();

    println!("Total cost for purchasing 3 laptops from each brand is: ₦{}", total);
}