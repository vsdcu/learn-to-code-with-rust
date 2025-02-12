fn main() {
    let number = 1_3_3_7;

    println!("number is {number}");

    let number_in_i16 = number as i16;

    println!("{number_in_i16}");

    let float_number = 3.1455654643439;

    println!("3 decimal precision number: {float_number:.3}");

    let with_milk = true;
    let with_sugar = true;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    println!("is_my_type_of_coffee: {is_my_type_of_coffee:?}");
    println!("is_acceptable_coffee: {is_acceptable_coffee:?}");

    let num_array: [i8; 5] = [1, 2, 3, 4, 5];

    println!("array-> {num_array:?}");
    dbg!(num_array);

    for num in num_array {
        println!("number in array is {num}");
    }

    let mix_tuple = (5, 3.14, true, num_array);

    dbg!({ mix_tuple });

    println!("mix_tuple: {mix_tuple:?}");
    println!("mix_tuple prettyprint: {mix_tuple:#?}");
}
