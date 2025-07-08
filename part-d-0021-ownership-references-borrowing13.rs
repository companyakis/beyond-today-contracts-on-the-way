fn main() {

    let sentence = "When I last saw her, she was 12.".to_string();

    //print_val(sentence);

    // println!("{sentence}") // Error => borrow of moved value: `sentence`

    print_val_ref(&sentence);

    print_val(sentence);
}

fn print_val(val: String) {

    println!("The value is here: {val}")
}

fn print_val_ref(val: &String) {

    println!("The value is here: {val}")
}
