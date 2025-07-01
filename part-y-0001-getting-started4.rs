/*
❓ Question 4: Using variable shadowing in Rust
Write a Rust program that demonstrates variable shadowing:

Declare a variable x and assign it a value.
Shadow x by declaring it again with a new value.
Print both values of x (before and after shadowing) to the terminal.

*/

fn main() {

    let x: u16 = 2025;

    println!("{x}");

    let x = "Mustafa";

    println!("{x}")
    
}

