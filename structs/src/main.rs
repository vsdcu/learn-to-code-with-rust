#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32,) -> Self {
            Flight {
                origin,
                destination,
                price,
                passengers,
            }
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination;
    }

    fn increase_price(&mut self) {
        self.price *=1.20;
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {

    let mut flight = Flight::new(String::from("Dublin"), 
            String::from("London"), 
            250.0, 
            1);

    println!("Original flight info:: {:#?}", flight);

    flight.change_destination(String::from("Galway"));
    flight.increase_price();

    println!("Updated flight info:: {:#?}", flight);

    flight.itinerary();

    let mut new_flight = Flight {
        origin: String::from("Galway"),
        destination: flight.destination.clone(),
        ..flight
    };

    println!("new_flight info:: {:#?}", new_flight);
    
    new_flight.change_destination(String::from("Paris"));
    new_flight.itinerary();

}


