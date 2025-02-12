fn main() {

    let mut cereals = [String::from("Cookie Crisp"),
                                String::from("Cinnamon Toast Crunch"),
                                String::from("Frosted Flakes"),
                                String::from("Cocoa Puffs"),
                                String::from("Captain Crunch")];
    
    println!("{cereals:?}");
    
    let first_two = &cereals[..2];
    println!("first_two: {first_two:?}");
    
    let mid_three = &cereals[1..4];
    println!("mid_three: {mid_three:?}");
    
    let last_three = &mut cereals[2..];
    println!("last_three: {last_three:?}");
    
    last_three[2] = String::from("Lucky Charms");
    
    println!("{cereals:?}");
    
    
    let cookie_crisp = &cereals[0];
    println!("{cookie_crisp}");
    
    let cookie = &cookie_crisp[..7];
    println!("cookie slice: {cookie}");
    
    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("puffs slice: {puffs}");
    
    }
    