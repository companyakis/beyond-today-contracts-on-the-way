/*
❓ Question 13: Defining an array and accessing its elements
Write a Rust program that does the following:

Define an array of five integers.
Print each element of the array to the terminal.
🔧 Task: Demonstrate array creation and element access in Rust

*/

fn main() {

    let ages: [u8; 5] = [9, 99, 77, 11, 83];

    for age in ages {

        println!("Age : {age}")
    }

}

