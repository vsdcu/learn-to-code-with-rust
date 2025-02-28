use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String;
    fn stats(&self) -> String{
        format!("{}", self.get_data())
    }
}

#[derive(Debug)]
pub enum Milk {
    Whole,
    Oat,
    Almond,
}

#[derive(Debug)]
pub struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T: Display> Coffee<T> {
    pub fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self {
            kind,
            milk,
            ounces,
        }
    }
}