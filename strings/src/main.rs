use std::{fmt, io};

fn make_money(input: &mut String) {
    input.push_str("$$$");
    println!("{input}");
}

fn trim_and_capitalize(slice: &str) -> String {
    slice.trim().to_uppercase()
}

fn elements (slice: &str) -> Vec<&str>{
    let output= slice.split("!").collect();
    return output;
}

fn get_identity() -> String {
    println!("Enter first name.");
    let mut first_name = String::new();

    let reader = io::stdin();
    reader.read_line(&mut first_name).expect("Failed to read first name!");
    //first_name = first_name.trim().to_string();

    println!("Enter last name.");
    let mut last_name = String::new();
    reader.read_line(&mut last_name).expect("Failed to read last name!");
    //last_name = last_name.trim().to_string();

    //let full_name = format!("{first_name} {last_name}");
    let full_name = format!("{} {}", first_name.trim(), last_name.trim());
    return full_name;

}

fn main() {
    make_money(&mut "Euro".to_string());
    let message = trim_and_capitalize("how to make money?");
    println!("{message}");
    let output = elements("Gold!Silver!Platinum");
    println!("Vector of {:?}", output);

    let name  = get_identity();
    println!("{name}");
}
