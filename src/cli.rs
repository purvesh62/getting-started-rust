pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    let command = args[1].clone();
    
    println!("Command: {}", command);

    if command == "greet" {
        println!("Hi, how are you?");
    } else if command == "name" {
        println!("Hi, I'm rusty.");
    } else {
        println!("Command not found.")
    }


    println!("Args: {:?}", args);
}