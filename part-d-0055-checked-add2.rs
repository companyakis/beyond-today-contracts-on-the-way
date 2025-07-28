fn main() {

    let a = u8::checked_add(11, 90);

    println!("{:?}", a);

    let b: u8 = 111;

    let c = u8::checked_add(b, 200);

    println!("{:?}", c);

}

// Some(101)
// None
