use std::vec;

/*
Vectors are resizable arrays
*/
pub fn run() {
    let mut vector: Vec<i32> = vec![1,2,3,4];

    // Add value to the vector 
    vector.push(2);

    // Pop value from the vector
    vector.pop();

    // Vector length
    println!("Length: {}", vector.len());

    // Loop through vector
    for i in vector.iter() {
        println!("Number: {}", i);
    }

    // Loop & mutate values
    for i in vector.iter_mut() {
        *i = *i * 2;
    }

    // Print vector
    println!("{:?}", vector);
}

