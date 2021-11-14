/**
 * Arrays -  has fixed list where elements are of the same types
 *
 */

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 4, 5, 6];
    // Resign value
    numbers[3] = 20;
    println!("{:?}", numbers);
    // Get single value
    println!("{:?}", numbers[0]);

    // loop through vectors

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // modify smillar to array.map is js
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers: {:?}", numbers);
}
