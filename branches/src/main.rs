use std::io;

fn main() {
    let number: i32 = take_input();

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn take_input() -> i32 {
    let mut input = String::new();
    
    println!("Please enter a number to see if it's smaller than 5:");

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input string into an integer
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            take_input()
        }
    };

    number
}