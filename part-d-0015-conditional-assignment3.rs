fn main() {

    let weekly_sales_usd: u16 = 14_250;

    let weekly_cost_usd: u16 = 9500;

    if weekly_sales_usd > 15_000 && weekly_cost_usd < 7_500 {

        println!("Awesome!")
    } 
    
    else if weekly_sales_usd > 10_000 || weekly_cost_usd < 5_000 {

        println!("We can achieve!")
    }

    else {

        println!("We need a new plan!")
    }

}
