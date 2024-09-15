#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    println!("rect1 is {:?}", rect1);
    println!("The area of the rect1 is {}", rect1.area());

    let rect2 = Rectangle { width: 40, ..rect1 };
    println!("rect2 is {:?}", rect2);
    println!("The area of the rect2 is {}", rect2.area());

    println!("rect1 can hold rect2 is {}", rect1.can_hold(&rect2));
}
