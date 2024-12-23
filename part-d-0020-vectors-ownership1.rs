fn main() {

    let years: Vec<u16> = vec![1923, 2020, 2000, 2054, 2024, 1990, 2018];

    let ceo = "Mustafa".to_string();
    let cfo = "Aygün".to_string();
    let chro = "Bengü".to_string();

    let people = vec![ceo, cfo, chro];

    //println!("Ceo: {ceo}"); // Error => value borrowed here after move

    let mustafa = &people[0];

    println!("Ceo: {mustafa}"); // Ceo: Mustafa

    let year_first = years[0];

    println!("{year_first}");

    let years_second = &years[1];

    println!("{years_second}");

}

// Ceo: Mustafa
// 1923
// 2020


