use std::collections::HashMap;

fn main() {

    let mut favorite_proverbs: HashMap<&str, &str> = HashMap::new();

    let person = String::from("Mustafa");

    let proverb = String::from("A rolling stone gathers no moss!");

    favorite_proverbs.insert(&person, &proverb);

    favorite_proverbs.insert("Aygün", "A barking dog never bites.");

    println!("{:?}", favorite_proverbs); // {"Aygün": "A barking dog never bites.", "Mustafa": "A rolling stone gathers no moss!"}
 
}

