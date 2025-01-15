fn main() {

    let sum_ages = |a: u8, b: u8, c: u8| a + b + c;

    let sum_family_ages = sum_ages(54, 78, 100);

    println!("Sum of family members ages: {sum_family_ages}"); // Sum of family members ages: 232

}

