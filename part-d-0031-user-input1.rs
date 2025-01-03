use std::io;

fn main() {

    let mut proverb = String::new();

    println!("What is your favorite proverb?");

    io::stdin()
        .read_line(&mut proverb)
        .expect("Failed to collect the user input!");

    println!("Your favorite proverb: {proverb}")

}


// What is your favorite proverb?
// A rolling stone gathers no moss
// Your favorite proverb: A rolling stone gathers no moss
