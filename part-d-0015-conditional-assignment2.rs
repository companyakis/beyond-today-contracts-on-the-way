fn main() {

    let is_completed = false;

    let msg: &str = if is_completed {
        "The process completed. You can go!"
    } else {
        "You have to wait!"
    };

    println!("The system message is: {msg}")

}

