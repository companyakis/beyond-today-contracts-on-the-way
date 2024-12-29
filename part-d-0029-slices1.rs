fn main() {

    let who_am_i = "Mustafa Büyükdereli".to_string();

    let my_name = &who_am_i[0..=6];

    println!("My first name is {my_name}.");

    let my_surname = &who_am_i[7..who_am_i.len()];

    println!("My first name is {my_surname}.");

}

// My first name is Mustafa.
// My first name is  Büyükdereli.

