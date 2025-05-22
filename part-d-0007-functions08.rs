fn main() {

  let january_sales_usd: f32 = 15_400.0;

  let february_sales_usd: f32 = 8_750.21;

  println!("January sales and premium: {:?}", sales_premiums(january_sales_usd)); // January sales and premium: (15400.0, 2310.0)

  println!("February sales and premium: {:?}", sales_premiums(february_sales_usd)); // February sales and premium: (8750.21, 437.5105)

 
}


fn sales_premiums(sales: f32) -> (f32, f32) {

    let mut premium: f32 = 0.0;

    if sales > 10_000.0 {
        premium = sales * 0.15
    } else if sales < 10_000.0 && sales > 0.0 {
        premium = sales * 0.05
    } else {
        premium = 0.0
    }

    (sales, premium)
}
