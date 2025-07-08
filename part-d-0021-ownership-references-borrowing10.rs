fn main() {

    let year: u16 = 2025; // stack val

    let name = String::from("Mustafa"); // heap val

    let area= "FinTech";

    let proverb: String = "Out of sight, out of mind!".to_string();

    let current_year = year;

    let current_year_reference = &year;

    println!("Year: {year}");

    println!("Current Year: {current_year}");

    println!("Current Year Ref: {current_year_reference}");

    println!("Name: {name}");

    let _first_name = name;

    //println!("Name: {name}"); // Error => borrow of moved value: `name`

    let fav_area = area;

    println!("Fav area: {fav_area}");

    println!("Area: {area}");

    println!("My proverb: {proverb}");

    // let a_proverb = proverb;

    // println!("My proverb: {proverb}"); // Error => value borrowed here after move

    let _good_proverb = &proverb;

    println!("My proverb: {proverb}");

}
