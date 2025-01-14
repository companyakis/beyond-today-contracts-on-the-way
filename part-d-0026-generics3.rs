fn main() {

    // turbofish operator

    println!("{}", return_me::<u16>(2025));

    println!("{}", return_me::<&str>("Hi!"));

    println!("{}", return_me::<String>("Mustafa".to_string()));

    println!("{}", return_me::<bool>(true));

    println!("{}", return_me::<f32>(3.14));

    println!("{:?}", return_me::<MyStruct>(MyStruct {}));

}

fn return_me<T>(i: T) -> T {
    i
}

#[derive(Debug)]
struct MyStruct {

}

// 2025
// Hi!
// Mustafa
// true
// 3.14
// MyStruct


