fn main() {

    let grades: Vec<(&str, u8)> = vec![
        ("Mustafa", 100),
        ("Aygün", 97),
        ("Bilge", 99)
    ];

    println!("Mustafa final grade: {:?}",grades[0].1) // Mustafa final grade: 100
}

