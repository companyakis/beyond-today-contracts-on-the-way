/*
In Rust, there are two commonly used types of arrays. One is the static array, directly allocated in stack memory, 
providing fast access but with a fixed length. The other is the dynamic array Vector, allocated in heap memory, 
allowing dynamic growth but with a performance cost.
*/

fn main() {

    let years: [u16;5] = [1990, 2013, 2016, 2025, 1923];

    let fives: [u8; 5]= [5; 5];

    let ones = [1; 7]; // i32

    println!("{years:?}");

    println!("{fives:?}");

    println!("{ones:?}");

}

// [1990, 2013, 2016, 2025, 1923]
// [5, 5, 5, 5, 5]
// [1, 1, 1, 1, 1, 1, 1]
