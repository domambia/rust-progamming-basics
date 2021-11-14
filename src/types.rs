/**
 * PRIMITIVE TYPES
 * - Integers:
 *            - Unsigned: u8,u16,u32,u64
 *            - Signed: i8, i16, i32,i64
 * Floats: f32, f64
 * Boolean: bool
 * Characters: char
 * Tuples
 * Arrays
 *  > Rust is a statically typed language, which means  that it must have the type of all
 *  > variables at compile time, however, the complier can usually infer what type we want
 *  > to use  based on the value and how we use it.
 */
pub fn run() {
    println!("************* TYPES START *************");
    // Default is "i32"
    let x = 1;

    //  Default is "f64"
    // let y  = 2.5;

    // Add explicit type
    let y: f64 = 1213213131.0;

    // Find Max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    let is_greater: bool = 10 > 12;

    println!("{:?}", (x, y, is_active, is_greater));

    let a1 = "\u{1F600}";

    println!("{}", a1);

    println!("************* TYPES END *************");
}
