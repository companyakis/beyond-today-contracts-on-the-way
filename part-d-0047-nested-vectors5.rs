fn main() {

    let mut yearly_quarterly_sales: Vec<Vec<u64>> = vec![vec![650_000, 870_000, 750_000]];

    yearly_quarterly_sales.push(vec![912_000, 456_000, 765_000]);

    println!("Current sales data: {:?}", yearly_quarterly_sales); // Current sales data: [[650000, 870000, 750000], [912000, 456000, 765000]]
}

