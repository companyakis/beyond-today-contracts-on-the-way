fn main() {

    sum(12, -25);

    diff(321, 213);

// 12 + -25 = -13
// 321 - 213 = 108

}

fn sum(a: i32, b: i32) {
    println!("{a} + {b} = {}", a + b)
}

fn diff(a: i32, b: i32) {
    println!("{a} - {b} = {}", a - b)
}
