macro_rules! calculate_profit_or_loss {

    ($sales_amount: expr, $cost: expr) => {
        
        $sales_amount - $cost
    };
}

fn main() {

    let profit_or_loss1 = calculate_profit_or_loss!(5000, 6250);

    println!("Profit/Loss amount 1: {profit_or_loss1}"); // Profit/Loss amount 1: -1250

    let profit_or_loss2 = calculate_profit_or_loss!(6400 as f32, 5800.12);

    println!("Profit/Loss amount 2: {profit_or_loss2}"); // Profit/Loss amount 2: 599.8799

}



