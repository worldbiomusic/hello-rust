#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn isWidthBigger(&self, other: &Rectangle) -> bool {
        self.width > other.width
    }
}

 
fn main() {
    let rect = Rectangle {
        width: 5, height: 6,
    };

    println!("area of the rect is {}", area(&rect));
    println!("rect: {:#?}", rect); // debug 형태 json 호출

    // rect.width()는 호출하는 rect가 &self로 들어가므로 "(&self).width()" (&self가 auto referencing 변환)
    // self자체를 넘기면 소유권을 move하므로 진척이 안됨
    println!("width is bigger than zero? {}", rect.width());

    let rect2 = Rectangle{
        width: 3, height: 5
    };
    // &self가 아닌 인자는 auto referencing 불가능
    println!("is rect width bigger then rect2 width? {}", rect.isWidthBigger(&rect2));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}