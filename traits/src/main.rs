mod challenge;
use challenge::{Coffee, Milk};

use traits::lodging::{Accommodation, AirBnB, Description, Hotel};
use traits::utils;

fn main() {

    let mut hotel = Hotel::new("Reddisson");
    println!("{}",hotel.get_description());
    hotel.book("Dave", 2);
    println!("hotel price : {}", hotel.total_price());
    hotel.set_base_price(20);
    println!("hotel new price : {}", hotel.total_price());
    utils::book_for_one_night(&mut hotel, "George");
    


    let mut airBnB = AirBnB::new("Peter");
    println!("{}",airBnB.get_description());
    airBnB.book("Tom", 2);
    println!("airBnB price : {}", airBnB.total_price());
    airBnB.set_base_price(2);
    println!("airBnB new price : {}", airBnB.total_price());
    utils::book_for_one_night(&mut airBnB, "Maya");
   

    utils::mix_and_match(&mut hotel, &mut airBnB, "Alex");

    // dynamic binding example
    // if underline methods are mutating we pass the mutable ref as "&mut dyn"
    let stays: Vec<&dyn Description> = vec![&hotel, &airBnB];
    println!("dyn-> {}", stays[0].get_description());
    println!("dyn-> {}", stays[1].get_description());

    println!("{hotel:#?}");
    println!("{airBnB:#?}");
}
