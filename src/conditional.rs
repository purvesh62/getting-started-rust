pub fn run() {
    let age = 24;
    let check_id = false;
    let knows_person_or_age = true;


    if age >= 21 && check_id || knows_person_or_age {
        println!("Bartender: What would you like to drink?")
    } 
    else if age < 21 && check_id {
        println!("Bartender: Sorry no drinks for you.")
    } else {
        println!("No ID no drinks.")
    }


    // Shorthand IF
    let is_of_age = if age >= 21 {true} else {false};

    println!("Is of Age: {}", is_of_age);
}