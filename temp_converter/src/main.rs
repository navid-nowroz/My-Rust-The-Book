use std::io::{self, Write};

const FTC: &str = "Fahrenheit to Celsius";
const CTF: &str = "Celsius to Fahrenheit";

fn main() {
    let mode = fahrenheit_or_celsius();
    println!("Selected mode: {mode}");
    match mode.trim() {
        FTC => {
            let temperature: &64 = get_temp();
            fahrenheit_to_celsius(temperature);
        },
        CTF => {
            let temperature: &64 = get_temp();
            celsius_to_fahrenheit(temperature);
        }
    }
}

// Gets the mode to run the program on
fn fahrenheit_or_celsius() -> &'static str{
    // Printing the header info
    println!("This is a simple rust based app to convert temperature developed by Twoki.");
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
                "1" => FTC,
                "2" => CTF,
                _ => unreachable!(),
            };
        } else {
            println!("Invalid input. Please try again.");
        }
    }
}

// Gets the temperature to convert from
fn get_temp() -> f64 {
    let mut input = String::new();
    let number: f64;
    println!("Give the temperature you want to convert from as input");

    loop {
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => {
                number = num;
                break;
            }
            Err(_) => println!("Invalid temperature. Please enter a valid decimal number."),
        }
    }
    number
}