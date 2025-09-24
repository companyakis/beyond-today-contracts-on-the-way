fn main() {

    let citizen_aysel = Customers::None;
    println!("{:?}", citizen_aysel);

    let old_customer1 = Customers::Old(String::from("Kayahan Kaya")); 
    println!("{:?}", old_customer1); // Old("Kayahan Kaya")

    if let Customers::Old(n) = old_customer1 {

        println!("{n}") // Kayahan Kaya
    }

    let new_customer1 = Customers::Current("Aykan Yakın".to_string());

    if let Customers::Current(n) = new_customer1 {

        println!("{n}") // Aykan Yakın
    } 
    
    // What happens if we want to use variable "n"? I'll cover...
}

#[derive(Debug)]
enum Customers {

    None,
    Old(String),
    Current(String)
}



    
