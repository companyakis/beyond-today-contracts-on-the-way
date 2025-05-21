fn main() {

    let powers_of_negative_3 = number_powers(-3);

    println!("{:?}", powers_of_negative_3) // [9, -27, 81]

}

fn number_powers(x: i64) -> Vec<i64> {

    vec![x * x, x * x * x, x * x * x * x]
}
