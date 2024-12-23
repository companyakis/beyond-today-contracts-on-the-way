fn main() {

    let years: Vec<u16> = vec![1923, 2020, 2000, 2054, 2024, 1990, 2018];

    let ceo = "Mustafa".to_string();
    let cfo = "Aygün".to_string();
    let chro = "Bengü".to_string();

    let people = vec![ceo, cfo, chro, "Hakan".to_string(), String::from("Ayhan")];

    // slice

    let people_slice = &people[2..=4];

    println!("{people_slice:?}");

    let years_slice = &years[2..5];

    println!("{years_slice:?}");
}

// ["Bengü", "Hakan", "Ayhan"]
// [2000, 2054, 2024]


