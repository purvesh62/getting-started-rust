pub fn run(){

    greeting("Hello", "Tia");

    let get_sum = add(1,2);
    println!("Sum = {}", get_sum);


    // Closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(4,6));

}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you.", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // Without semicolon it'll directly return the statement
    n1 + n2
}
