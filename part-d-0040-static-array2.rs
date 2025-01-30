fn main() {
  
    let quarterly_sales_by_month: [[u16; 3]; 4] = [
        [5700, 12400, 8750],
        [6400, 7500, 8200],
        [5800, 9000, 11000],
        [6900, 9650, 10500],
    ];

    println!("Quarterly sales by month in 2024: {quarterly_sales_by_month:?}")
}

// Quarterly sales by month in 2024: [[5700, 12400, 8750], [6400, 7500, 8200], [5800, 9000, 11000], [6900, 9650, 10500]]
