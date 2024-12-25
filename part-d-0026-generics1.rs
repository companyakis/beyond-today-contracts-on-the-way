fn main() {

    println!("{}", return_me(22.67));

    println!("{}", return_me(true));

    println!("{}", return_me("Mustafa"))

}

fn return_me<T>(me: T) -> T { me }

// 22.67
// true   
// Mustafa

