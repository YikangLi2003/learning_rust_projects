struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    // &self has exact same meaining of self: &Self i.e. immutable reference
    fn width(&self) -> bool {
        self.width > 0
    }

    fn set(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let const_r1 = Rectangle {
        width: 10,
        height: 20
    };

    println!("{} {} {}", const_r1.width, const_r1.height, const_r1.width());

    // cosnt_r1 is an immutable variable so it cannot be borrowed as mutable reference by it's method.
    // const_r1.set(20, 40);

    let mut mutable_r2 = Rectangle {
        width: 20,
        height: 20,
    };

    println!("mutable_r2: {} {}", mutable_r2.width, mutable_r2.height);

    mutable_r2.set(40, 40);
    
    println!("mutable_r2: {} {}", mutable_r2.width, mutable_r2.height);

    let r2_ref: &Rectangle = &mutable_r2;

    // r2_ref.width is exactly same as (&r2_ref).width
    // Rust will autimatically deference to the actual object
    println!("immutable r2 refference: {} {}", r2_ref.width, (&r2_ref).height);

    let small_rect = Rectangle {
        width: 10,
        height: 20,
    };

    let medium_rect = Rectangle {
        width: 20,
        height: 30,
    };

    let large_rect = Rectangle {
        width: 30,
        height: 40,
    };

    println!("Large rectangle can hold small rectangle: {}", large_rect.can_hold(&small_rect));
    println!("Medium rectangle can hold small rectangle: {}", medium_rect.can_hold(&small_rect));
    println!("Small rectangle can hold large rectangle: {}", small_rect.can_hold(&large_rect));
    println!("Medium rectangle can hold large rectangle: {}", medium_rect.can_hold(&large_rect));
}