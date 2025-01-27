use std::mem::size_of_val;

fn main() {

    let first_letter_of_name = 'M';

    let current_year: u16 = 2025;

    let year: i32 = 2025;

    // let's test memory consumptions

    println!("Size 1: {}", size_of_val(&first_letter_of_name));

    println!("Size 2: {}", size_of_val(&current_year));

    println!("Size 3: {}", size_of_val(&year));

}

// Size 1: 4
// Size 2: 2
// Size 3: 4
