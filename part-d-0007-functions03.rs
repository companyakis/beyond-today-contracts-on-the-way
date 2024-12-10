fn main() {

    let sum_result = sum(312, 214);

    println!("Sum result: {sum_result}");

    let diff_result = diff(12.47, -3 as f32);

    println!("Difference result: {diff_result}");

}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn diff(a: f32, b: f32) -> f32 {
    a - b
}

// Sum result: 526
// Difference result: 15.47
