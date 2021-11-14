// Reference Pointer - Points to a resource in memory;

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3, 5];
    let arr2 = arr1;

    println!("values: {:?}", (arr1, arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You will need to use a reference(&) to point to resource

    // Vector;

    let vec1 = vec![1, 2, 3, 5];
    let vec2 = &vec1;
    println!("Vectors: {:?}", (&vec1, vec2));
}