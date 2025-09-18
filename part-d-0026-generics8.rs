fn main() {

    let a: u16 = 2025;

    let b: f32 = 3.14;

    // Shadowing

    let (a, b) = swap_nums(a, b);

    println!("a: {a}"); // a: 3.14

    println!("b: {b}"); // b: 2025

}

fn swap_nums<T1, T2>(x: T1, y: T2) -> (T2, T1) {

    (y, x)
}



