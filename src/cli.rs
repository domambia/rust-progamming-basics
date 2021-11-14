use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    println!("Args: {:?}", args);
    println!("Command: {:?}", command);
    let mut name = "Omambia";
    if command == "target=World" {
        name = "Coding";
    }

    println!("What is the name?: {}", name);
}
