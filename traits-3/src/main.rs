use std::fmt::{Debug, Display, Formatter, Result};


enum AppleType {
    GrannySmith,
    RedDelicious,
}

struct Apple {
    kind: AppleType,
    price: f64
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Apple :::: [kind:{:?}  price:{}]", self.kind, self.price)
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
