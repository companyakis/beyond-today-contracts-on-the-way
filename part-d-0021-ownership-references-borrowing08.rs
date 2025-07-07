fn main() {

    let my_name = String::from("Mustafa was here!");

    let who_are_you = &my_name;

    println!("{my_name}");

    println!("{who_are_you}");

    println!("The memory address of my_name is {:p}", who_are_you);

    let mut greeting = String::from("Hello");

    let add_name = &mut greeting;

    add_name.push_str(", Mustafa!");

    //println!("{greeting}"); // cannot borrow `greeting` as immutable because it is also borrowed as mutable

    println!("{add_name}");
    
}

// Mustafa was here!
// Mustafa was here!
// The memory address of my_name is 0xc490d8f6b0
// Hello, Mustafa!
