mod macros_tut;
mod serde_tut;
mod borsh;
// traits are the properties that struct wants to be in it. like rect is the struct that has the trait of shape why? because rect has the area function if it does not have this function then rect does not have the trait of shape. similary for circle also.
trait Shape {
    fn area(&self) -> u32;
}

struct Rect {
    width: u32,
    height: u32
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
}

struct Circle {
    radius: u32
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        return self.radius * self.radius;
    }
}

fn main() {
    let r: Rect = Rect {
        width: 10,
        height: 10
    };

    let c: Circle = Circle { radius: (5) };

    get_area(r);
    get_area(c);

    macros_tut::macros();
    serde_tut::serde_eg();
    borsh::borsh();
}

// this is the function that only accepts the things that has the trait of shape, rect and circle both has that because both of them has area function in them.
fn get_area(s: impl Shape) -> u32 {
    return s.area()
}