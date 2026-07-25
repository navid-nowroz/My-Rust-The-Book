fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The are of a rectangle is {} square pixels.", area(width1, height1));
}

fn area(width: u64, height: u64) -> u64 {
    width * height
}