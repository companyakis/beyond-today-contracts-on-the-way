fn main() {

    // push and push_str

    let mut who_am_i = "Mustafa".to_string();

    let my_lastname = "Büyükdereli".to_string();

    who_am_i.push(' ');

    who_am_i.push_str(&my_lastname);

    println!("Who am I? : {who_am_i}...") // Who am I? : Mustafa Büyükdereli...

}

