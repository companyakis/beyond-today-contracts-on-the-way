fn main() {

    let monthly_sales: u16 = 54_000;

    let monthly_premium: f32 = if monthly_sales > 50_000 {

        monthly_sales as f32 * 0.128

    } else if monthly_sales > 40_000 {

        monthly_sales as f32 * 0.08

    } else {

        0.0
    };

    println!("Current premium: ${monthly_premium}") // Current premium: $6912.0005

}




