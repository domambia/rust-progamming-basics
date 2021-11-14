// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    println!("*************** VARIABES START ********************");
    let name = "Omambia";

    println!("My name is: {}", name);

    // let mut age = 27;

    let age = 26;
    println!("My name {} and my age is: {}", name, age);
    // DEFINE constants
    const ID: i32 = 001;
    println!("MY ID: {}", ID);
    let (my_name, my_age) = ("Omambia", 37);
    println!("My name {} and my age is: {}", my_name, my_age);
    println!("************* VARIABLES END ************");
}
