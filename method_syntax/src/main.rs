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

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    println!("area is: {}", rect1.area());

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };
    println!("can hold?: {}", rect1.can_hold(&rect2));

    let square1 = Rectangle::square(10);
    println!("square is {:#?}", square1);
}
