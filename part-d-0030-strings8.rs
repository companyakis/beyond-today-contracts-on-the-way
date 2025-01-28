fn main() {
    
    let me = String::from("Mustafa Buyukdereli");

    let first_name: &str = &me[0..7];

    let last_name: &str = &me[7..me.len()];

    println!("{first_name} -{last_name}") // Mustafa - Buyukdereli
}

