pub fn run(){
    
    let mut count = 0;

    loop {
        count+=1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While loop with (FizzBuzz)
    while count <=100 {
        if count%15==0 {
            println!("FizzBuzz")
        } else if count%3 ==0 {
            println!("fizz")
        } else if count%5==0 {
            println!("buzz")
        } else {
            println!("{}", count)
        }
        count+=1;
    }

    // For range
    for i in 0..100 {
        if i%15==0 {
            println!("FizzBuzz")
        } else if i%3 ==0 {
            println!("fizz")

        } else if i%5==0 {
            println!("buzz")

        } else {
            println!("{}", i)
        }
    }

}