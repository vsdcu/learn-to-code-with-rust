use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;

enum AppleType {
    GrannySmith,
    RedDelicious,
}

struct Apple {
    kind: AppleType,
    price: f64
}

// drop function implementation will be called automatically when objects go out of scope.
impl Drop for Apple {
    fn drop(&mut self) {
        println!("Drop function called for {}", self.kind);
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Apple file deleted successfuly!"),
            Err(error) => eprintln!("Some issue while deleting apple file, {}", error),
        }
    }
}

impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::GrannySmith => write!(f, "🍏 Granny Smith apples.. 🍏🍏🍏"),
            AppleType::RedDelicious => write!(f, "🍎 Red Delicious apples.. 🍎🍎🍎"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::GrannySmith => write!(f, "AppleType::GrannySmith"),
            AppleType::RedDelicious => write!(f, "AppleType::RedDelicious"),
        }
    }
}

impl Display for Apple {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
       write!(f, "{} @ {} ", self.kind, self.price)
   } 
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        //write!(formatter, "Apple :::: [kind:{:?}  price:{}]", self.kind, self.price)
        formatter.debug_struct("** Apple **")
        .field("kind", &self.kind)
        .field("price", &self.price)
        .finish()
    }
}

fn main() {

    let green_apples = Apple{kind: AppleType::GrannySmith, price: 2.02};
    println!("{}", green_apples);
    println!("{:?}", green_apples);

    let red_apples = Apple {kind: AppleType::RedDelicious, price: 2.52};
    println!("{}", red_apples);
    println!("{:?}", red_apples);

}
