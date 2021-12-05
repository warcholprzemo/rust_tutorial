struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // The simple method, with only 1 arg - self (&Self)
    fn circumference(&self) -> u32 {
        (self.width + self.height) * 2
    }
    // method that has more arguments
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // "static" method - assicoated function, we can use it to create
    // a new rectangle (square in this case)
    // we call it using ::, Rectangle::square(70)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

pub fn main_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 25,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The circumference of the rectangle is {} pixels.",
        rect1.circumference()
    );
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    let square = Rectangle::square(70);
    println!("Square has width {} and height {}", square.width, square.height);

    // naive approach
    // let width1 = 30;
    // let height1 = 50;
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );
    // fn area(width: u32, height: u32) -> u32 {
    //     width * height
    // }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
