type Sales = u64;

type Rate = f32;

fn main() {

    let june_sales: Sales = 650_000;

    let cost_ratio: Rate = 0.72;

    println!("June gross profit: {}", june_sales as f32 * (1.0 - cost_ratio)) // June gross profit: 181999.98

}



