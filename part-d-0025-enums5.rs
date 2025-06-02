fn main() {

    let mut phone1 = Product::new("X Model".to_string(), ProductsPriceLevel::High(650_f32));

    phone1.activate_sales();

    println!("{:?}", phone1); // Product { name: "X Model", level: High(650.0), on_sale: true }

    phone1.price_change(ProductsPriceLevel::Medium(420.35));

    phone1.deactivate_sales();

    println!("{:?}", phone1); // Product { name: "X Model", level: Medium(420.35), on_sale: false }

}

#[derive(Debug)]
enum ProductsPriceLevel {
    High(f32),
    Medium(f32),
    Low(f32)
}

#[derive(Debug)]
struct Product {
    name: String,
    level: ProductsPriceLevel,
    on_sale: bool
}

impl Product {
    fn new(name: String, level: ProductsPriceLevel) -> Self {
        Product { name, level, on_sale: false }
    }

    fn price_change(&mut self, level: ProductsPriceLevel) {
        self.level = level
    }

    fn activate_sales(&mut self) {
        self.on_sale = true
    }

    fn deactivate_sales(&mut self) {
        self.on_sale = false
    }
}

