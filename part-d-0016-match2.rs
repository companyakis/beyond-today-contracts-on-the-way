fn main() {

    let result: u8 = 4;

    let is_passed: bool = match result {

        1..=3 => false,
        4..=5 => true,
        _ => false
        
    };

    println!("Is passed: {}", is_passed)

}

