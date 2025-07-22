fn main() {

    let mut yearly_quarterly_sales: Vec<Vec<u64>> = vec![vec![650_000, 870_000, 750_000]];

    yearly_quarterly_sales.push(vec![912_000, 456_000, 765_000]);

    yearly_quarterly_sales.insert(2, vec![690_000, 587_000, 654_000]);

    yearly_quarterly_sales.push(vec![812_000, 356_900, 565_600]);

    let quarter1_sales_2024 = yearly_quarterly_sales.get(12);

    match quarter1_sales_2024 {
        
        Some(s) => println!("2024 quarter 1 sales data: {:?}", s),
        None => println!("No available sales data"),
    }

    let quarter1_sales_2017 = yearly_quarterly_sales.get(1);

    match quarter1_sales_2017 {
      
        Some(s) => println!("2017 quarter 1 sales data: {:?}", s),
        None => println!("No available sales data"),
    }
}

// No available sales data
// 2017 quarter 1 sales data: [912000, 456000, 765000]
