#[derive(Debug)]
struct Rectangle {
    height: usize,
    width: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width 
    }
}
fn main() {
    let rect: Rectangle    = Rectangle {
        height: 30,
        width: 50
    };
    
    let rect2 = Rectangle {
        height: 10,
        width: 40,
    };

    let rect3 = Rectangle {
        height: 60,
        width: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
