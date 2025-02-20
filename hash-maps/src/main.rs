use std::collections::HashMap;

fn main() {

    //creation of map with from construct
    //from takes [] of tuples
let mut sauces_to_meals= HashMap::from(
    [
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"])
    ]
);

//insert
sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);
println!("{sauces_to_meals:#?}");

//remove (unwrap the option)
let result = sauces_to_meals.remove("Mayonnaise").unwrap();
println!("removed vector values: {:?}", result);

//get (match the option)
let get_result = sauces_to_meals.get("Mustard");
match get_result {
    Some(v) => println!("get vector values: {:?}", v)
    None => println!("key not founf in the map!")
}


//entry and or_insert
sauces_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings"]);
println!("{:#?}", sauces_to_meals);

}
