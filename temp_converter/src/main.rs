use std::io;

fn main() {
    let mode = Fahrenheit_Or_Celsius();
    println!("Selected mode: {mode}");
}

fn Fahrenheit_Or_Celsius() {
    // Printing the header info
    println!("This is a simple rust based app to convert temperature developed by Twoki.")
    println!("Give the number 1 as input if you want to convert from Fahrenheit to Celsius.");
    println!("Give the number 2 as input if you want to convert from Celsius to Fahrenheit.");

    // getting the starting variables set up
    let mut input = String::new();
    let valid_inputs = vec!["1", "2"];
    // getting the loop ready to take input untill correct input detected
    loop {
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed_input = input.trim();
        if valid_inputs.contains(&trimmed_input) {
            return match trimmed_input {
                "1" => "Fahrenheit to Celsius",
                "2" => "Celsius to Fahrenheit",
                _ => unreachable!(),
            };ß
        } else {
            println!("Invalid input. Please try again.");
        }
    }
}
