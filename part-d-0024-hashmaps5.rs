use std::collections::HashMap;

fn main() {

    let mut favorite_proverbs: HashMap<&str, &str> = HashMap::new();

    let person = String::from("Mustafa");

    let proverb = String::from("A rolling stone gathers no moss!");

    favorite_proverbs.insert(&person, &proverb);

    favorite_proverbs.insert("Aygün", "A barking dog never bites.");

    // get values by keys

    println!("{:?}", favorite_proverbs["Mustafa"]); // "A rolling stone gathers no moss!"

    //println!("{:?}", favorite_proverbs["Ayhan"]); // Error! => no entry found for key

    // get copied unwrap_or

    let mustafa_proverb = favorite_proverbs.get("Mustafa").copied().unwrap_or("This person/proverb doesn't exist!");

    println!("Mustafa's proverb: {}", mustafa_proverb); // Mustafa's proverb: A rolling stone gathers no moss!

    let ayhan_proverb = favorite_proverbs.get("Ayhan").copied().unwrap_or("This person/proverb doesn't exist!");

    println!("Ayhan's proverb: {}", ayhan_proverb) // Ayhan's proverb: This person/proverb doesn't exist!
 
}

