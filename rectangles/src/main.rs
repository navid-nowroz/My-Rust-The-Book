fn main() {
    struct Rectangle {
        height: usize,
        width: usize,
    }

    let rect1 = Rectangle {
        height: 30,
        width: 50
    };

    println!("The are of a rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: Rectangle) -> usize {
    rectangle.height * rectangle.width
}