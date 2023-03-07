// Variables are immutable by default.

pub fn run() {
    // immutable by default
    let name = "Purvesh";
    println!("My name is {}", name);

    // Define a mutable variable 
    let mut age = 24;
    println!("Age is {}", age);

    age = 25;
    println!("Age is now {}", age);

    // When using const you HAVE TO add variable type 
    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (my_name, my_age) = ("Purvesh", 24);
    println!("My name is {} and age {}", my_name, my_age);
}