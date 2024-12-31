fn main() {

    // unwrap_or

    let year1 = Some(100).unwrap_or(0);

    let year2 = None.unwrap_or(0);

    println!("{:?}", year1); // 100

    println!("{:?}",year2); // 0

}

