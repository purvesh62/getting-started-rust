/*
RUST 

Primitive types :-

Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 
FLOAT: f32, f64
Boolean: bool
Characters: char
Tuples: ()
Arrays: [] #Fixed length


Non-primitive types :-


*/

pub fn run() {
    // Default int: i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 12312312312;

    // Find max size of i32
    println!("Max i32: {}", std::i32::MAX);

    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;


    // Character
    let ch = 'a';

    println!("{:?}", (x,y,z,ch,is_active));

}