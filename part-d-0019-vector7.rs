fn main() {

    let mut sales_months: Vec<u8> = Vec::with_capacity(12);

    println!("Vector length: {} and vector capacity: {}", sales_months.len(), sales_months.capacity());

    sales_months.push(1);
    sales_months.push(2);
    sales_months.push(3);
    sales_months.push(4);
    sales_months.push(5);
    sales_months.push(6);
    sales_months.push(7);
    sales_months.push(8);
    sales_months.push(9);
    sales_months.push(10);
    sales_months.push(11);
    sales_months.push(12);

    println!("Vector length: {} and vector capacity: {}", sales_months.len(), sales_months.capacity());

    // now let's add new sales month
    // heap, new data area, pointer...

    sales_months.push(13);
    sales_months.push(14);

    println!("Vector length: {} and vector capacity: {}", sales_months.len(), sales_months.capacity());

    // Vector length: 0 and vector capacity: 12
    // Vector length: 12 and vector capacity: 12
    // Vector length: 14 and vector capacity: 24

}






