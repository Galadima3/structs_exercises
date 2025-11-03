struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}
impl Rectangle {
    fn create_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

struct Student {
    name: String,
    id: u32,
}
impl Student {
    fn output_details(&self) -> String {
        format!("Name: {}, ID: {}", self.name, self.id)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 25,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 40,
    };

    let mut student = Student {
        name: String::from("Jamie"),
        id: 2050,
    };

    student.name = String::from("Rogan");



    let square_rect: Rectangle = Rectangle::create_square(20);

    println!("Area of Rectangle = {}", rect1.area());
    println!("Area of Square Rect = {}", square_rect.area());
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
    println!("Student Details => {}\n", student.output_details());
}
