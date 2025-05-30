use std::f64::consts::PI;

// Define the Shape trait with methods for area and perimeter calculation
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    
    // Default method to return the shape's name
    fn name(&self) -> &str {
        "Unknown Shape"
    }
}

// Implement Circle
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
    
    fn name(&self) -> &str {
        "Circle"
    }
}

// Implement Rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn name(&self) -> &str {
        "Rectangle"
    }
}

// Implement Triangle
struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

impl Triangle {
    fn new(side_a: f64, side_b: f64, side_c: f64) -> Option<Self> {
        if side_a + side_b > side_c && 
           side_b + side_c > side_a && 
           side_a + side_c > side_b {
            Some(Self { side_a, side_b, side_c })
        } else {
            None
        }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }
    
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
    
    fn name(&self) -> &str {
        "Triangle"
    }
}

// Function to print shape information using trait bounds
fn print_shape_info(shape: &dyn Shape) {
    println!("Shape: {}", shape.name());
    println!("Area: {:.2}", shape.area());
    println!("Perimeter: {:.2}", shape.perimeter());
    println!();
}

fn print_shape_collection(shape: &dyn Shape) {
    println!("{} - Area: {:.2}, Perimeter: {:.2}", 
        shape.name(), 
        shape.area(), 
        shape.perimeter()
    );
}

fn main() {
    // Create instances of each shape
    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(3.0, 5.0);
    let triangle = Triangle::new(3.0, 4.0, 5.0).expect("Invalid triangle sides");

    // Print information for each shape
    print_shape_info(&circle);
    print_shape_info(&rectangle);
    print_shape_info(&triangle);

    // Store shapes in a vector of trait objects
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(circle),
        Box::new(rectangle),
        Box::new(triangle),
    ];

    println!("Shapes in collection:");
    for shape in &shapes {
        print_shape_collection(shape.as_ref());
    }
}