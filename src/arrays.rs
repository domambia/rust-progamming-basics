/**
 * Arrays -  has fixed list where elements are of the same types
 *
 */

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 4, 5, 6];
    // Resign value
    numbers[3] = 20;
    println!("{:?}", numbers);
    // Get single value
    println!("{:?}", numbers[0]);
}
