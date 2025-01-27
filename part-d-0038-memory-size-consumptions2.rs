use std::mem::size_of_val;

fn main() {

    let my_name: &str = "Mustafa";

    let my_real_name: String = String::from("Mustafa");

    let years1 = [2023, 2024, 2025];

    let years2: [u16; 3] = [2023, 2024, 2025];

    let years3: Vec<u16> = vec![2023, 2024, 2025];

    // let's test memory consumptions

    println!("Size 1: {}", size_of_val(&my_name));

    println!("Size 2: {}", size_of_val(&my_real_name));

    println!("Size 3: {}", size_of_val(&years1));

    println!("Size 4: {}", size_of_val(&years2));

    println!("Size 5: {}", size_of_val(&years3));    
}

// Size 1: 16
// Size 2: 24
// Size 3: 12
// Size 4: 6 
// Size 5: 24
