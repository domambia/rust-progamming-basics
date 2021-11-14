pub fn run() {
    // Pring to console
    println!("************* PRINT & FORMATING START *************");

    let name = "Omambia Mogaka Dauglous";

    // BASIC formating
    println!("{} is from {}", name, "Mars");
    // Positional Arguments
    println!(
        "{0} is from {1} and  {0} likes to {2}",
        name, "Mars", "code"
    );

    println!(
        "{name} likes to play {activity}",
        name = name,
        activity = "Base Ball"
    );

    // placeholders
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder debug traits
    println!("{:?}", (12, true, "Hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
    println!("************* PRINT & FORMATING END *************");
}
