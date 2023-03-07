// Array - fixed list

use std::mem;


pub fn run() {
    // [i32 -> type; 5 -> length]
    let mut numbers: [i32; 3] = [1,2,3];

    println!("{:?}", numbers);

    // Get single value
    println!("{:?}", numbers[0]); 

    // Re-assign values
    numbers[2] = 20;

    // Get array length
    println!("Array length {}", numbers.len());

    // Array stack allocated memory
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);

}