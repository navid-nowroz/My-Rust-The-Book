use std::io;

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn take_input() -> i32 {
    let mut input = String::new();

    println!("please enter the number here to see if it's smaller than 5");

    io::stdin().read_line(&mut input)
        .expect("Please enter a valid integer");

    let input: i32 = input.trim().parse();
    input
}