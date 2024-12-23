fn main() {

    let result: u8 = 4;

    let is_passed: &str = match result {

        1 | 2 => "Bad result. Not passed!",
        3 => "Average result. Passed!",
        4 | 5 => "Good result. Thanks!",
        _ => "Invalid query!"
        
    };

    println!("Is passed?: {}", is_passed)

}

