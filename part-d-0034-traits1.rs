fn main() {

}

trait Geometry {

    fn calculate_area(&self) -> f64;

    fn calculate_perimeter(&self) -> f64;
}

struct Rectangular {
    width: f64,
    height: f64
}

struct Circular {
    radius: f64
}

impl Geometry for Rectangular {

    fn calculate_area(&self) -> f64 {
        self.height * self.width
    }

    fn calculate_perimeter(&self) -> f64 {
        2.0 * (self.height + self.width)
    }
}

impl Geometry for Circular {

    fn calculate_area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }

    fn calculate_perimeter(&self) -> f64 {
        2.0 * 3.14 * self.radius
    }
}





