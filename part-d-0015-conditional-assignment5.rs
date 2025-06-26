fn main() {

    let january_sales: u32 = 42_500;

    println!("January sales report: {:?}", sales_result_report(january_sales)) // January sales report: "Good result!"
}

fn sales_result_report(monthly_sale: u32) -> String {

    let result = if monthly_sale > 50_000 {

        String::from("Awesome sales! Thanks...")

    } else if monthly_sale > 25_000 {

        String::from("Good result!")

    } else {

        String::from("We should try")
    };

    result
}
