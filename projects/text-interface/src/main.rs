use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    println!("Valid commands are: ");
    println!("Add Doe to Engineering -> to add Doe to Engineering department.");
    println!("Engineering            -> to see the list of staff in Engineering department");
    println!("Quit                   -> to quit the program. \n");

    let mut company :HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let command :String = get_string(String::from("Enter your command here: "));
        let command_list :Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

        match command_list.len() {
            1 => {
                let depth_or_action :&String = &command_list[0];

                if depth_or_action.to_lowercase() == "quit" {
                    println!("Exiting program. Goodbye...");
                    break;
                } else if company.contains_key(depth_or_action) {
                    println!("Staff in {depth_or_action} : {:?}", company.get(depth_or_action).unwrap());
                } else {
                    println!("Department {depth_or_action} does not exist or has no staff yet.");
                }
            },

            4 => {
                let action :&String = &command_list[0];
                let employee :&String = &command_list[1];
                let to_keyword :&String = &command_list[2];
                let department :&String = &command_list[3];

                if action.to_lowercase() == "add" && to_keyword.to_lowercase() == "to" {
                    company.entry(department.to_string()).or_insert(Vec::new()).push(employee.to_string());
                    println!("Added {employee} to {department} department.");
                } else {
                    println!("Invalid command format. Use: Add [Employee] to [Department]");
                }
            }
            _ => {
                println!("Command not recognized. Please follow the instructions.");
            }
        }
        println!();
    }
}


fn get_string(prompt :String) -> String {
    println!("{prompt}");
    if io::stdout().flush().is_err() {
        return get_string(prompt);
    }

    let mut input :String = String::new();

    match io::stdin().read_line(& mut input) {
        Ok(_) => {
            let cleaned :String = input.trim().to_string();
            let is_valid :bool = cleaned.chars().all(|c| c.is_alphabetic() || c == ' ');

            match (cleaned.is_empty(), is_valid) {
                (true, _) => {
                    println!("Input cannot be empty. Try again...");
                    get_string(prompt)
                },
                (_, false) => {
                    println!("Invalid input. Only English letters and spaces are allowed. Try again...");
                    get_string(prompt)
                },
                (false, true) => cleaned,
            }
        },
        Err(_) => {
            println!("Failed to read line. Retrying....");
            get_string(prompt)
        }
    }
}