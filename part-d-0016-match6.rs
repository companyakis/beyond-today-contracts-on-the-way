fn main() {

    println!("Premium report 1: {:?}", premium_result_report(37_350)) // Premium report 1: 2614.5

    
}

fn premium_result_report(monthly_sale: u32) -> f32 {

    match monthly_sale {
        
        50_001..=100_000 => monthly_sale as f32 * 0.15,

        25_000..=50_000 => monthly_sale as f32 * 0.07,

        _ => 0.0
        
    }

}
