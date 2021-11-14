/**
 * Tuples grpoup togethers value  of different types;
 * max 12 elements
 */
pub fn run() {
    let person: (&str, &str, i8) = ("Omambia", "Kids", 26);

    println!("{} has  2 {} and he is: {}", person.0, person.1, person.2);
}
