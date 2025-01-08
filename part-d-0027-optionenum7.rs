fn main() {

    let condition1 = are_sales_enough(120_000, 65_000);

    //let condition2 = are_sales_enough(140_000, 45_000);

    match condition1 {

        Option::Some(t) => println!("Are sales enough 1: {t}"),
        Option::None => println!("Unexpected result... Look at the data"),
    }
}

fn are_sales_enough(sales1: u32, sales2: u32) -> Option<bool> {

    if sales1 > 100_000 && sales2 > 50_000 {
        Option::Some(true)

    } else if sales1 < 100_000 || sales2 < 50_000 {

        Option::Some(false)

    } else {
        Option::None
    }
}


