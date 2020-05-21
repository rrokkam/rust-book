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
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rect is {:#?}", rect);
    println!("Rect's area = {}", rect.area());

    let square = Rectangle::square(30);

    let another_rect = Rectangle {
        width: 60,
        height: 45,
    };

    println!("can rect hold square? {}", rect.can_hold(&square));
    println!("can rect hold another_rect? {}", rect.can_hold(&another_rect));
}

