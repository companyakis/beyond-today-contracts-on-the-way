fn main() {

    let mut name = String::from("Mustafa");

    let first_name = name.clone();

    println!("{:?}", hello_adding(&mut name)); // "Mustafa, welcome! Nice to meet you!"

    println!("My name is {}!", first_name) // My name is Mustafa!
}

fn hello_adding(name: &mut String) -> &mut String {

    name.push_str(", welcome! Nice to meet you!");

    name
}

