use std::io;

fn main() {
    let input :String = get_string();
    let pig :String = piggify(&input);
    println!("{pig}");
}

fn get_string() -> String {
    println!("Please enter the string that you want to convert here. ");
    let mut input :String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn piggify(string :&String) -> String {
    let list :Vec<char> = string.chars().collect();
    let vowels :Vec<char>= vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    if vowels.contains(&list[0]) {
        let sentence :String = list.into_iter().collect();
        let sentence:String = sentence + "-hay";
        sentence
    } else {
        let slice:&[char] = &list[1..];
        let pig :String = slice.into_iter().collect();
        let some :String = list[0].to_string();
        let pig:String = pig + "-" + &some + "ay";
        pig
    }
}