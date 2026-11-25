use std::io;
use std::collections::HashMap;

fn main() {
    let vector: Vec<i32> = take_vector();
    let median :i32 = get_median(&vector);
    let mode :i32 = get_mode(&vector);
    println!("Median: {median}, Mode: {mode}");

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
    data.sort();
    let length :i32 = data.len() as i32;

    let median :i32 = match length % 2 {
        0 => {
            let med_point :usize = (length/2) as usize;
            let result :i32 = (data[med_point - 1] + data[med_point]) / 2;
            result
        },
        _ => {
            let med_point :usize = ((length - 1)/2) as usize;
            data[med_point]
        }
    };
    median
}

fn get_mode(vector :&Vec<i32>) -> i32 {
    let data :Vec<i32> = vector.clone();
    let mut map:HashMap<i32, i32> = HashMap::new();

    for number in &data {
        let count = map.entry(*number).or_insert(0);
        *count += 1;
    }

    let mut big :i32 = 0;
    let mut return_value :i32 = 0;
    for (key, value) in map {
       if value > big {
           big = value;
           return_value = key;
       } else {
           continue;
       }
    }
    return_value
}