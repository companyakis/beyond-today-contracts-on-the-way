fn main() {

    let mut sales_by_emp_ayhan_sales = SalesByEmployee::new("Ayhan Bilir".to_string(), "Senior Expert".to_string(), YearsAndSales(2021, 45_550_f32));

    println!("Emp Ayhan sales data: {:?}", sales_by_emp_ayhan_sales);

    sales_by_emp_ayhan_sales.update_sales_data(2022, 65_321_f32);

    println!("Emp Ayhan sales data: {:?}", sales_by_emp_ayhan_sales);

    // Emp Ayhan sales data: SalesByEmployee { name: "Ayhan Bilir", title: "Senior Expert", sales_data: YearsAndSales(2021, 45550.0) }
    // Emp Ayhan sales data: SalesByEmployee { name: "Ayhan Bilir", title: "Senior Expert", sales_data: YearsAndSales(2022, 65321.0) }

}

#[derive(Debug)]
struct YearsAndSales(u16, f32);

#[derive(Debug)]
struct SalesByEmployee {
    name: String,
    title: String,
    sales_data: YearsAndSales
}

impl SalesByEmployee {
    
    fn new(name: String, title: String, sales_data: YearsAndSales) -> Self {

        SalesByEmployee { name, title, sales_data }
    }

    fn update_sales_data(&mut self, new_year: u16, new_sales_amount: f32) {

        self.sales_data.0 = new_year;
        self.sales_data.1 = new_sales_amount;
    }

}


