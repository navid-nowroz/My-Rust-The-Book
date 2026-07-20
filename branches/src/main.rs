use std::io;

fn main() {
    println!("Please a number to see if it's less than 5");
    let number: i32 = get_number();

    if number < 5 {
        println!("The number is less than 5.");
    } else {
        println!("The number is not less than 5");
    }
}

fn get_number() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Please enter a valid integer");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            get_number()
        }
    };
    number 
}