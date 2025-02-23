use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self);
    fn book(&mut self, guest_name:String, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: std::collections::HashMap<String, u32>,
}

impl Hotel {
    fn new(name: String) -> Self {
        Self {
            name: name, 
            reservations: HashMap::new(),
        }
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) {
        println!("Welcome to the hotel {}", self.name);
    }

    fn book(&mut self, guest_name:String, nights: u32) {
        self.reservations.insert(guest_name, nights);
    }
}


fn main() {

    let mut hotel = Hotel::new("Reddisson".to_string());
    hotel.get_description();
    hotel.book(String::from("Dave"), 2);

    println!("{hotel:#?}");

}
