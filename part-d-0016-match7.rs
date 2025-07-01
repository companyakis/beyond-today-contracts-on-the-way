fn main() {

    let age: u8 = 21;

    match age {
        _a if _a % 2 == 0 => println!("{_a} is even!"),

        _a if _a % 2 != 0 => println!("{_a} is odd!"),

        _ => println!("")

    }

}

