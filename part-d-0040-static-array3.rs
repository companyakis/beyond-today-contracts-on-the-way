fn main() {

    let my_chars: [char; 7] = ['M', 'u', 's', 't', 'a', 'f', 'a'];

    for c in my_chars {
        print!("{c}") 
    }

    println!();

    println!("Last char: {}", my_chars[my_chars.len() - 1])

}

// Mustafa
// Last char: a
