use std::{collections::HashMap, fmt::Display};

pub trait Accommodation {
    fn book(&mut self, guest_name:&str, nights: u32);
}

pub trait Description {
    fn get_description(&self) -> String;
}


#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: std::collections::HashMap<String, u32>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name, 
            reservations: HashMap::new(),
        }
    }
}

impl<T> Accommodation for Hotel<T> {
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
    host: String,
    guests: Vec<(String, u32)>,
}


impl AirBnB {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, guest_name:&str, nights: u32) {
        self.guests.push((guest_name.to_string(), nights));
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Welcome to the AirBnB of {}", self.host)
    }
}