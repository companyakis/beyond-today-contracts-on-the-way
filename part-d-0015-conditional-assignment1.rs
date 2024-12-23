fn is_even_or_odd(number: i64) {
    let result = if number % 2 == 0 { "even"} else { "odd" };

    println!("The number {number} is {result}.")
}

fn main() {

    is_even_or_odd(123);

    is_even_or_odd(6578)

}

// The number 123 is odd.
// The number 6578 is even.
