fn main() {

    println!("{}", is_even_or_bigger_50(12));

    println!("{}", is_even_or_bigger_50(51));

    println!("{}", is_even_or_bigger_50(-5));

    println!("{}", is_even_or_bigger_50(212));

}

fn is_even_or_bigger_50(a: i64) -> bool {
    
     if a % 2 == 0 || a > 50 { true }

     else { false }
}

// true
// true 
// false
// true 
