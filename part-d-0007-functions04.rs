fn main() {
    let result1 = first_last_chars("Aziz");

    println!("{}", result1); // true

    let result2 = first_last_chars("Kültigin");

    println!("{}", result2) // false
}

fn first_last_chars(text: &str) -> bool {
  
    if text.to_lowercase().chars().nth(0).unwrap() == 'a' && text.to_lowercase().chars().nth(text.len() - 1).unwrap() == 'z' { true }

    else { false}
     
}
