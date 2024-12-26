use std::collections::HashMap;

fn main() {

    let mut favorite_proverbs: HashMap<&str, &str> = HashMap::new();

    let person = String::from("Mustafa");

    let proverb = String::from("A rolling stone gathers no moss!");

    favorite_proverbs.insert(&person, &proverb);

    favorite_proverbs.insert("Aygün", "A barking dog never bites.");

    // entry method -> if the key doesn't exist, it works...

    favorite_proverbs.entry("Aygün").or_insert("Lorem ipsum");

    println!("{:?}", favorite_proverbs);

    favorite_proverbs.entry("Kültigin").or_insert("Kurt kışı geçirir, yediği ayazı unutmaz!");

    println!("{:?}", favorite_proverbs);

}

// {"Aygün": "A barking dog never bites.", "Mustafa": "A rolling stone gathers no moss!"}
// {"Aygün": "A barking dog never bites.", "Kültigin": "Kurt kışı geçirir, yediği ayazı unutmaz!", "Mustafa": "A rolling stone gathers no moss!"}
