use std::fmt;

pub fn run() {
    let minmax = MinMax(8, 128);
    println!("Display: {}", minmax);
    println!("Debug: {:?}\n", minmax);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display: {}", point);
    println!("Debug: {:?}\n", point);

    let complex = ComplexNumber { real: 3.1, imag: 4.2 };
    println!("Display: {}", complex);
    println!("Debug: {:?}\n", complex);
}

#[derive(Debug)]
struct ComplexNumber {
    real: f32,
    imag: f32
}

impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
