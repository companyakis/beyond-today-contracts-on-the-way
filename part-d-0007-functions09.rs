fn main() {

    let sales_last_year: [u32; 12] = [45000, 17000, 21000, 36000, 43000, 42000, 65000, 39000, 82000, 54000, 28000, 57000];

    let premiums_of_last_year = premium_calculator(sales_last_year);

    println!("{:?}", premiums_of_last_year) // [6750.0005, 0.0, 0.0, 0.0, 6450.0005, 6300.0005, 9750.0, 0.0, 12300.001, 8100.0005, 0.0, 8550.0]
 
}


fn premium_calculator(monthly_sales: [u32; 12]) -> Vec<f32> {

    let mut sales_results = Vec::new();

    for sale in monthly_sales {

        if sale > 40_000 {
            sales_results.push(sale as f32 * 0.15)
        } 

        else {
            sales_results.push(0.0)
        }
    }

    sales_results
}
