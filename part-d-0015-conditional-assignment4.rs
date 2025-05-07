fn main() {

    let weekly_sales_usd1: u16 = 11_000;

    println!("Week 1 premium: {}", premium_calculator(weekly_sales_usd1 as f32));

    let weekly_sales_usd2: u16 = 8_200;

    println!("Week 2 premium: {}", premium_calculator(weekly_sales_usd2 as f32));
}


fn premium_calculator(sales: f32) -> f32 {

    let premium = if sales > 10_000.0 {sales * 0.12} else {0.0};

    premium
}

// Week 1 premium: 1320
// Week 2 premium: 0
