fn main() {
    apply_to_jobs(35, "Rust Developer");

    let result = is_even(8);
    println!("Result: {result}");

    let result = is_even(9);
    println!("Result: {result}");

    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("zebra"));

    let value = nested_function();
    println!("Final value: {value}");
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number:?} {title} jobs");
}

fn is_even(number: i32) -> bool {
    return number % 2 == 0;
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains('a'), text.contains('z'))
}

fn nested_function() -> i32 {
    let first_value = 10;

    let multiply = {
        let second_value = 2;
        first_value * second_value
    };

    println!("here is first value: {first_value}");
    return multiply;
}
