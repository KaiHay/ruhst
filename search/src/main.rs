use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => PI * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
    let circle: Shape = Shape::Circle(10.0);
    let rectangle: Shape = Shape::Rectangle {
        width: (10.0),
        height: (5.0),
    };
    let triangle: Shape = Shape::Triangle(5.0, 5.0, 5.0);

    println!(
        "Circle area: {}, Rectangle area: {}, Triangle Area: {}",
        circle.area(),
        rectangle.area(),
        triangle.area()
    )
}
