fn main() {

    let total_balance;

    let budget_month;

    {
        let user_balance: f32 = 1250.89;

        let month = String::from("January");

        total_balance = &user_balance;

        budget_month = &month;

        println!("Total balance: {total_balance} and budget month: {budget_month}") // Total balance: 1250.89 and budget month: January
    }

    //println!("Total balance: {total_balance} and budget month: {budget_month}") // if we run, we get "borrowed value does not live long enough"

}

