fn main() {

    let a = u8::checked_add(11, 90).unwrap_or(0);

    println!("{:?}", a);

    let b: u8 = 111;

    let c = u8::checked_add(b, 200).unwrap_or(0);

    println!("{:?}", c);

}

// 101
// 0
