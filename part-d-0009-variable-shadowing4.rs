fn main() {

    let div: f32 = 12.0 / 7.0;

    println!("Division result: {div}");

    let div: f64 = 12.0 / 7.0;

    println!("Division result: {div}");

    println!("Smoothing values: {}, {}, {}", div.ceil(), div.floor(), div.round())

}

// Division result: 1.7142857
// Division result: 1.7142857142857142
// Smoothing values: 2, 1, 2



