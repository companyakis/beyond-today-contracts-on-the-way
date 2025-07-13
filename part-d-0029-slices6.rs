fn main() {

    let mut six_month_sales: [u16; 6] = [15_400, 14_650, 21_000, 12_300, 15_700, 16_420];

    let quarter_2_sales = &mut six_month_sales[3..];

    // update sales amounts

    quarter_2_sales[0] = 16_400;
    quarter_2_sales[1] = 18_690;
    quarter_2_sales[2] = 21_440;

    println!("Six month sales (updated amounts): {:?}", six_month_sales) // Six month sales (updated amounts): [15400, 14650, 21000, 16400, 18690, 21440]
    
}

