struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function: does not have &self as first argument.
    // Often used to create and return a new object of associated struct.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    // Methods
    
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 10,
        height: 20,
    };

    // Use :: syntax to call associated function of a struct
    let s1 = Rectangle::square(5);

    println!("r1 can hold s1: {}", r1.can_hold(&s1));
    println!("r1 area = {}", r1.area());
    println!("s1 area = {}", s1.area());
}