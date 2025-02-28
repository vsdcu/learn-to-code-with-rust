use std::{collections::HashMap, fmt::Display};

pub trait Accommodation {
    const GOVT_TAX_MULTIPLIER: u32 = 10;
    fn book(&mut self, guest_name:&str, nights: u32);

    // getter method 
    fn get_base_price(&self) -> u32;

    // setter pattern to modify the struct's state/field
    fn set_base_price(&mut self, new_base_price: u32);

    // default impl in trait,
    fn total_price(&self) -> u32 {
        self.get_base_price() * Self::GOVT_TAX_MULTIPLIER
    }
}

pub trait Description {
    fn get_description(&self) -> String;
}


#[derive(Debug)]
pub struct Hotel<T> {
    price: u32,
    name: T,
    reservations: std::collections::HashMap<String, u32>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            price: 10,
            name, 
            reservations: HashMap::new(),
        }
    }
}

impl<T> Accommodation for Hotel<T> {

    // getter pattern using trait for returning the struct state/field.
    fn get_base_price(&self) -> u32 {
        self.price
    }

    fn set_base_price(&mut self, new_base_price: u32) {
        self.price = new_base_price;
    }

    fn book(&mut self, guest_name:&str, nights: u32) {
        self.reservations.insert(guest_name.to_string(), nights);
    }
}

impl<T: Display> Description for Hotel<T> {
    fn get_description(&self) -> String {
        format!("Welcome to the hotel {}", self.name)
    }
}

#[derive(Debug)]
pub struct AirBnB {
    price: u32,
    host: String,
    guests: Vec<(String, u32)>,
}


impl AirBnB {
    pub fn new(host: &str) -> Self {
        Self {
            price: 1,
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    
    // getter pattern using trait for returning the struct state/field.
    fn get_base_price(&self) -> u32 {
        self.price
    }

    // setter pattern to modify the struct's state/field
    fn set_base_price(&mut self, new_base_price: u32) {
        self.price = new_base_price;
    }

    fn book(&mut self, guest_name:&str, nights: u32) {
        self.guests.push((guest_name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Welcome to the AirBnB of {}", self.host)
    }
}