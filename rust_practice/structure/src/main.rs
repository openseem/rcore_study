#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        if self.width >= rec.width && self.height > rec.height {
            return true;
        }
        false
    }

    fn square(long: String) -> Rectangle {
        let p: u32 = long.trim().parse().expect("msg");
        Rectangle {
            width: p,
            height: p,
        }
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );
    rec1.area();
    println!("rec1 is {:#?}", rec1);
    let p = String::from("321312");
    Rectangle::square(p);
    Rectangle::square(p);
}
