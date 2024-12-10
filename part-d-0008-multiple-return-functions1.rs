fn main() {

    let calculation = sum_multiply(12, -5, 4);

    let sum = calculation.0;

    let multiplication = calculation.1;

    println!("Sum: {sum} and multiplication: {multiplication}") // Sum: 11 and multiplication: -240

}

fn sum_multiply(a: i64, b: i64, c: i64) -> (i64, i64) {

    (a + b + c, a * b * c)
}
