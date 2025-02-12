#[derive(Debug)]
struct Food {
    name: String
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        } 
        
        if self.reservations < 12 {
            Some(Food { name: String::from("Uni Shahsi")})
        } else {
            Some(Food { name: String::from("Strip Steak")})
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            Err("Sorry, we have a mice problem".to_string())
        } else if address.is_empty() {
            Err("No delivery address specified".to_string())
        } else {
            Ok(Food {name: "Burger".to_string()})
        }
    }

}


fn main() {

    let restuarant = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };

    let chef_special = restuarant.chef_special();
    println!("{:?}", chef_special);

    let deliver_burger = 
        restuarant.deliver_burger("123 Elm Street");
    println!("{:?}", deliver_burger);


    let restaurant_two = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    let chef_special = restaurant_two.chef_special();
    println!("{:?}", chef_special);

    let deliver_burger = 
    restaurant_two.deliver_burger("");
    println!("{:?}", deliver_burger);

    let deliver_burger = 
    restaurant_two.deliver_burger("123 Elm Street");
    println!("{:?}", deliver_burger);

}
