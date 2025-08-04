fn main() {

    let mut sales: Vec<u16> = vec![4200, 6700, 6240, 4582];

    sales.push(7000);

    let sales_october_result: Result<&u16, &'static str> = sales.get(9).ok_or("Index out of bounds");

    match sales_october_result {
        
        Ok(s) => println!("Ocotober sales amount: ${s}"),
        Err(e) => println!("No available sales data => {e}")
    }
}

// No available sales data => Index out of bounds
