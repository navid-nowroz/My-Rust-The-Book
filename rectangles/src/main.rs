#[derive(Debug)]
struct Rectangle {
    height: usize,
    width: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.height * self.width
    }
}
fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50
    };

    println!("The are of a rectangle is {} square pixels.", rect1.area());
}
