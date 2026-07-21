use std::io::(self, Write);

fn main() {
    println!("Give the nth position of the number in fibonacci series.");
    let mut n: i32 = get_number();
}


fn get_number() -> i32 {
    let mut input = String::new();
    let number: i32;

    io::stdout().flush().unwrap();
    input.clear();

    io::stdin().read_line(& mut input).expect("Failed to read line");
    number = match input.trim().parse::<i32> {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid integer.");
            get_number();
        }
    };
    number
}