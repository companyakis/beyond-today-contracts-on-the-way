fn main() {

    // format macro

    let my_firstname = "Mustafa".to_string();

    let my_lastname = "Büyükdereli".to_string();

    let who_am_i = format!("Mr. {0} {1}! Yes, my name is {0}...", my_firstname, my_lastname);

    println!("{who_am_i}");

    println!("{my_firstname}");

    println!("{my_lastname}")

}

// Mr. Mustafa Büyükdereli! Yes, my name is Mustafa...
// Mustafa    
// Büyükdereli
