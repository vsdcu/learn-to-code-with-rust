fn main() {
    let values = [4, 8, 15, 16, 23, 42];

    let regular_reference = &values;
    print_length(regular_reference);

    let slice_of_three = &values[..3];
    print_length(slice_of_three);
}

fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}
  