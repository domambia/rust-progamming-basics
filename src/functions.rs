pub fn run() {
    greetings("Hello", "Winnie");

    // Bind function value to a variable
    let get_sum = add(5, 6);
    println!("SUM: {}", get_sum);

    // Closure
    let n3: i32 = 16;
    let add_sum = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_sum(3, 4));
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}, nice to meet you.", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
    // return n1 + n2
}
