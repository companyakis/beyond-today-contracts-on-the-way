fn main() {

    let user_input1 = "150 kg";

    let user_input2 = "150";

    let parsed1 = user_input1.parse::<u8>();

    let parsed2 = user_input2.parse::<u8>();

    println!("Parsed data 1: {:?}", parsed1);

    println!("Parsed data 1: {:?}", parsed2);

    
    // Parsed data 1: Err(ParseIntError { kind: InvalidDigit })
    // Parsed data 1: Ok(150)

}




