/*
❓ Question 3: Difference between let and const in Rust
Write a Rust program that demonstrates the difference between a variable declared with let and a constant declared with const.

Declare a mutable variable x with let and assign it a value.
Declare a constant Y with const and assign it a value.
Change the value of x and print both x and Y to the terminal.

*/

const Y: i8 = -10;

fn main() {

    let mut x: u16 = 2000;

    x = 2006;

    println!("x: {x} and Y: {Y}") // x: 2006 and Y: -10
    
}

