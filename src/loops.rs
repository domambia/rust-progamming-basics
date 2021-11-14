pub fn run() {
    println!("*********** LOOPS ************");
    // let mut count = 1;
    // // infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);
    //     if count == 20 {
    //         break;
    //     }
    // }
    // count = 1;

    // While Loop (FizzBuz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         // both 3 & 5
    //         println!("Number: {}, FizzBuzz", count);
    //     }
    //     if count % 3 == 0 {
    //         println!("Number: {}, Fizz", count);
    //     }
    //     if count % 5 == 0 {
    //         println!("Number: {}, Buzz", count);
    //     }
    //     // Inc
    //     count += 1;
    // }

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            // both 3 & 5
            println!("Number: {}, FizzBuzz", x);
        }
        if x % 3 == 0 {
            println!("Number: {}, Fizz", x);
        }
        if x % 5 == 0 {
            println!("Number: {}, Buzz", x);
        }
    }
    println!("*********** END OF LOOPS ************");
}
