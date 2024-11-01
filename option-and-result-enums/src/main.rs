use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    coffee_pairings.entry("Latte").or_insert("Pistachio Milk");
    println!("{coffee_pairings:?}");
    // {"Flat White": "Almond Milk", "Latte": "Oat Milk"}
    coffee_pairings
        .entry("Cappuccino")
        .or_insert("Pistachio Milk");
    println!("{coffee_pairings:?}");
    // {"Cappuccino": "Pistachio Milk", "Latte": "Oat Milk", "Flat White": "Almond Milk"}
}
