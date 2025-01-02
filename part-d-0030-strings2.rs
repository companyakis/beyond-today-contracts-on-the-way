fn main() {

    // +

    let my_firstname = "Mustafa".to_string();

    let my_lastname = " Büyükdereli".to_string();

    let who_am_i = my_firstname + &my_lastname;

    println!("{who_am_i}"); // Mustafa Büyükdereli

    //println!("{my_firstname}"); // Error! => value borrowed here after move

}

