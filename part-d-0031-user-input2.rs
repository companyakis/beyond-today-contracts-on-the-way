use std::io;

fn main() {

    let mut proverb = String::new();

    println!("What is your favorite proverb?");

    match io::stdin().read_line(&mut proverb) {

        Ok(_) => println!("Your favorite proverb: {}", proverb.trim()),

        Err(msg) => println!("Error: {msg}")
    }  

}

// What is your favorite proverb?
//     Out of sight, out of mind
// Your favorite proverb: Out of sight, out of mind
