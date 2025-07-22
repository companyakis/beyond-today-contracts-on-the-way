fn main() {

    let mut yearly_quarterly_sales: Vec<Vec<u64>> = vec![vec![650_000, 870_000, 750_000]];

    yearly_quarterly_sales.push(vec![912_000, 456_000, 765_000]);

    yearly_quarterly_sales.insert(2, vec![690_000, 587_000, 654_000]);

    yearly_quarterly_sales.remove(1);

    println!("Current sales data: {:?}", yearly_quarterly_sales); // Current sales data: [[650000, 870000, 750000], [690000, 587000, 654000]]
}

