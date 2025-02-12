fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}

fn color_to_number_match(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn main() {
    let output = color_to_number("pink");
    println!("number for color is {output}");

    let output = color_to_number_match("blue");
    println!("number for color is {output}");

    let input = 5;
    let result = factorial(input);
    println!("factorial of {input} is {result}");

    let input = 5;
    let result = factorial_recursion(input);
    println!("factorial_recursion of {input} is {result}");

    let input = 5;
    let result = factorial_recursion_sol(input);
    println!("factorial_recursion_sol of {input} is {result}");
}

// normal function
fn factorial(mut number: i32) -> i32 {
    let mut result: i32 = number;

    while number != 1 {
        number -= 1;
        result = result * number;
    }

    return result;
}

// recursion function
fn factorial_recursion(mut number: i32) -> i32 {
    let mut result = number;

    while number != 1 {
        number -= 1;
        result = result * number;
        factorial_recursion(number);
    }

    return result;
}

// recursion function
fn factorial_recursion_sol(number: i32) -> i32 {

    if number == 1 {
      return 1;
    }
    number * factorial_recursion_sol(number - 1)
}