fn main() {

    // we have a new customer
    // we've learnt her/his name is different

    let mut new_customer1 = Customers::Current("Aykan Yakın".to_string());

    let mut new_name = String::new();

    if let Customers::Current(_) = new_customer1 {

        new_name = "Ayhan Bge Yakın".to_string();

        new_customer1 = Customers::Current(new_name.clone());

        println!("{:?}", new_customer1) // Current("Ayhan Bge Yakın")
    }

    new_name = "Ayhan Bilge Yakın".to_string();

    println!("{}", &new_name); // Ayhan Bilge Yakın


}

#[derive(Debug)]
enum Customers {

    None,
    Old(String),
    Current(String)
}
