fn main() {

    // let's define a customer

    let mut customer_aydin_ucar = Customer::new(String::from("Aydın Uçar"), String::from("701547"), String::from("23"), 6521.34, 0.15);

    let interest_calculated = customer_aydin_ucar.calculate_interest();

    println!("Total interest = {interest_calculated} ₺"); // Total interest = 978.201 ₺

    // let's change branch

    let new_branch= customer_aydin_ucar.branch_id = "45".to_string();

    println!("Customer info: {customer_aydin_ucar:?}") // Customer info: Customer { name: "Aydın Uçar", account_number: "701547", branch_id: "45", account_balance: 6521.34, private_interest_rate: 0.15 }

}

#[derive(Debug)]
struct Customer {
    name: String,
    account_number: String,
    branch_id: String,
    account_balance: f32,
    private_interest_rate: f32,
}

impl Customer {

    fn new(name: String, account_number: String, branch_id: String, account_balance: f32, private_interest_rate: f32) -> Self {

        Customer {name, account_number, branch_id, account_balance, private_interest_rate}

    }

    fn calculate_interest(&self) -> f32 {

        self.account_balance * self.private_interest_rate
    }
    
}

