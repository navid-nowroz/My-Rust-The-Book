use std::io;

fn main() {
    let mut vector: Vec<i32> = take_vector();
}


fn take_vector() -> Vec<i32> {
    println!("Enter the numbers separated by spaces: ");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    numbers
}

fn get_median(vector: &Vec<i32>) -> i32 {
    let mut data: Vec<i32> = vector.clone();
    let length :i32 = data.len() as i32;

    data.sort();
    80
}