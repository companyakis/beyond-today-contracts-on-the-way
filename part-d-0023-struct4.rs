fn main() {

    let tea_order = make_drink("Tea".to_string(), 11.25, 'm', true);

    println!("{}", tea_order.name);

}

struct Drink {
    name: String,
    price: f32,
    size: char, // 's', 'm', 'l
    hot: bool
}

fn make_drink(name: String, price: f32, size: char, hot: bool) -> Drink {
    Drink { name: name, price: price, size: size, hot: hot }
}
