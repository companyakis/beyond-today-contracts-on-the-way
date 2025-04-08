fn main() {

    let add_subtract_multiply = |a: i64, b: i64| {
        
        (a + b, a - b, a * b)
    };

    let result = add_subtract_multiply(11, 82);

    println!("a + b: {}", result.0);
    
    println!("a - b: {}", result.1);
    
    println!("a * b: {}", result.2)

}


// a + b: 93
// a - b: -71
// a * b: 902
