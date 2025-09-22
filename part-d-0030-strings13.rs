fn main() {

    let me = String::from("Mustafa");
    
    for (i, c) in me.char_indices() {
        
        println!("Index: {i} and char: {c}")
    }
}

// Index: 0 and char: M
// Index: 1 and char: u
// Index: 2 and char: s
// Index: 3 and char: t
// Index: 4 and char: a
// Index: 5 and char: f
// Index: 6 and char: a
