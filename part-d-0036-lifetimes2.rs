fn main() {

    let name = "Aydın Uçamaz".to_string();
    let id = String::from("701566");
    let balance: f32 = 5210.85;

    let customer_au = Customer {
        name: &name,
        id: &id,
        balance: balance
    };

    println!("Customer data: {:?}", customer_au) // Customer data: Customer { name: "Aydın Uçamaz", id: "701566", balance: 5210.85 }

}

#[derive(Debug)]
struct Customer<'a> {
    name: & 'a str,
    id: & 'a str,
    balance: f32,
}

// missing lifetime specifier
// expected named lifetime parameter
// struct Customer {
//     name: &str,
//     id: &str,
//     balance: f32,
// }
