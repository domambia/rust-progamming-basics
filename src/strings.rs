/**
 * TYPES
 *  1. Primitive == Immutable fixed-length string in memory
 *  2. String = Growable, heap allocated data structure- Use when you need to modity or own string data;
 */
pub fn run() {
    println!("********** STRINGS ************");
    let mut hello = String::from("Hello, ");
    println!("{}", hello);

    // get length
    println!("length: {}", hello.len());

    // Add char to String
    hello.push('W');

    // Push string
    hello.push_str("orld");
    println!("{}", hello);

    // Capacity

    println!("Capacity: {}", hello.capacity());

    // Is Empty
    println!("IS Empty: {}", hello.is_empty());

    // Contains
    println!("Contains: {}", hello.contains("World"));
    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion Test
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("********** END OF STRING ************");
}
