use std::fmt::Debug;

fn main() {

    let sales_quarter_1: QuarterlySales<u16> = QuarterlySales::new(5600, 4200, 6500);

    sales_quarter_1.quarterly_sales_info();

}


struct QuarterlySales<T: Debug> {

    month1: T,
    month2: T,
    month3: T
}

impl<T: Debug> QuarterlySales<T>  {
    
    fn new(month1: T, month2: T, month3: T) -> Self {

        QuarterlySales { month1, month2, month3 }
    }

    fn quarterly_sales_info(&self) {

        println!("Month1 sales amount: ${:?}", self.month1);
        println!("Month2 sales amount: ${:?}", self.month2);
        println!("Month3 sales amount: ${:?}", self.month3);
    }

}


