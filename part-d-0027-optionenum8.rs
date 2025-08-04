fn main() {

    let mut sales: Vec<u16> = vec![4200, 6700, 6240, 4582];

    sales.push(7000);

    let month_six_sales1 = sales.get(6).unwrap_or(&0);

    println!("June sales amount: ${:?}", month_six_sales1);

    let month_six_sales2 = sales.get(6);

    println!("June sales amount: ${:?}", month_six_sales2);
}

// June sales amount: $0
// June sales amount: $None
