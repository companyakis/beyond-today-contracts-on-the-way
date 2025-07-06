/*
Question 11: Defining a tuple and accessing its elements
Write a Rust program that does the following:

Define a tuple with three elements of different types.
Access and print each element of the tuple to the terminal.
🔧 Task: Demonstrate tuple creation and element access in Rust.

*/

fn main() {

    let my_info:(&'static str, u16, bool) = ("Mustafa Buyukdereli", 2025, true);

    println!("My info part 1: {}", my_info.0);
    println!("My info part 2: {}", my_info.1);
    println!("My info part 3: {}", my_info.2);

}

// My info part 1: Mustafa Buyukdereli
// My info part 2: 2025
// My info part 3: true
