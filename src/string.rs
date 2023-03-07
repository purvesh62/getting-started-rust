// Primitive str = Immutable fixed length string in memory
// String = Growable string, heap allocated, use when need to modify string data

pub fn run(){
    let mut hello = String::from("Hello");

    // Get length #works for all types
    println!("{}", hello.len());

    // push is to add #character
    hello.push(' ');
    hello.push('W');

    // push_str is to add string
    hello.push_str("orld");


    // Get string capacity in #bytes
    println!("{}", hello.capacity());


    // Check if empty
    println!("empty = {}", hello.is_empty());

    // Contains 
    println!("contains = {}", hello.contains("hey"));

    // Replace
    println!("replace = {}", hello.replace("H", "B"));


    // Loop through white space
    println!("Loop over whitespace");
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // Create string with capacity
    let mut str = String::with_capacity(10);
    str.push('a');
    str.push('b');

    // Assertion testing
    assert_eq!(2, str.len());

    println!("{}", hello);
}