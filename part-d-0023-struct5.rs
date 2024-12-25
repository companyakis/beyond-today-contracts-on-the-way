fn main() {

    let turkish_coffee = make_drink("Turkish Coffee".to_string(), 11.25, 's', true);

    let new_coffee = Drink {name: "My Coffee".to_string(), ..turkish_coffee}; // update name

}

struct Drink {
    name: String,
    price: f32,
    size: char, // 's', 'm', 'l
    hot: bool
}

fn make_drink(name: String, price: f32, size: char, hot: bool) -> Drink {
    Drink { name, price, size, hot}
}
