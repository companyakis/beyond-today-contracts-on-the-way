/*
❓ Question 5: Converting number types using as
Write a Rust program that does the following:

Declare a variable with a floating-point value.
Convert the value to an integer using the as keyword.
Print both the original and converted values to the terminal.
🔧 Task: Demonstrate type conversion in Rust using the as keyword.
*/

fn main() {

    let x: f32 = 3.14;

    println!("{x}");

    let x = x as u8;

    println!("{x}");
    
}

