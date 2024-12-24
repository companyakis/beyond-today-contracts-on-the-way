fn main() {

    // stack data

    let year = 1990; 

    let birth_year = year;

    let his_year = &year; // borrowing and referencing => NOT owner!

    // dereferencing

    let dereferenced_his_year = *his_year;

    println!("{dereferenced_his_year}"); //1990

}
