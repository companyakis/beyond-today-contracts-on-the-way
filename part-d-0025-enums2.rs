fn main() {

    let rect = GeoShapes::Rectangle(6 as f32, 8.25 );

    let squ = GeoShapes::Square(5.0);

    let circ = GeoShapes::Circle(3.25);

    println!("The area of a rectangle: {}", area_calculator(&rect));

    println!("The area of the square: {}", area_calculator(&squ));

    println!("The Area of the circle: {}", area_calculator(&circ));

// The area of a rectangle: 49.5
// The area of the square: 25      
// The Area of the circle: 33.16625

}

enum GeoShapes {
    Rectangle(f32, f32),
    Square(f32),
    Circle(f32)
}

fn area_calculator(geo_shape: &GeoShapes) -> f32 {

    match geo_shape {
        
        GeoShapes::Rectangle(width, height ) => width * height,
        GeoShapes::Square(i) => i * i,
        GeoShapes::Circle(radius) => 3.14 * radius * radius
    }
}
