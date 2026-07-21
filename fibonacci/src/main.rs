use std::io::{self, Write};

fn main() {
    println!("Give the nth position of the number in fibonacci series.");
    let n: i32 = get_number();
    let result: i32 = fibonacci(n);
    println!("The {n}th fibonacci number is {result}.");
}


fn get_number() -> i32 {
    let mut input = String::new();
    let number: i32;

    io::stdout().flush().unwrap();
    input.clear();

    io::stdin().read_line(& mut input).expect("Failed to read line");
    number = match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            get_number()
        }
    };
    number
}

fn fibonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut index = 0;
    
    while index < n {
        let temp = a + b;
        a = b;
        b = temp;
        index += 1;
    }
    a
}