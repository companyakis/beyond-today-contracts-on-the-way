fn main() {
    let result1 = return_longest_word("Lorem", "Hello", "Hi");

    println!("Result 1: {result1}"); // Result 1: Unfortunately, you should try again:)

    let result2 = return_longest_word("Corresponding", "Usage", "Lorem");

    println!("The longest is: {result2}"); // The longest is: Corresponding
}

fn return_longest_word<'a>(word1: &'a str, word2: &'a str, word3: &'a str) -> &'a str {
    if word1.len() > word2.len() && word1.len() > word3.len() {
        word1
    } else if word2.len() > word1.len() && word2.len() > word3.len() {
        word2
    } else if word3.len() > word1.len() && word3.len() > word2.len() {
        word3
    } else {
        "Unfortunately, you should try again:)"
    }
}
