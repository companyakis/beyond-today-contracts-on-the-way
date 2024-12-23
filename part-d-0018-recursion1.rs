fn factorial(num: u64) -> u64 {

    if num == 0 { 1 }

    else { num * factorial(num -1)}
}


fn main() {

    println!("Number: {} and factorial: {}", 7, factorial(7)) // Number: 7 and factorial: 5040

}


